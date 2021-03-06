use schema::flags::{FastFlag, IndexedFlag, SchemaFlagList, StoredFlag};
use std::ops::BitOr;

/// Express whether a field is single-value or multi-valued.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Cardinality {
    /// The document must have exactly one value associated to the document.
    #[serde(rename = "single")]
    SingleValue,
    /// The document can have any number of values associated to the document.
    /// This is more memory and CPU expensive than the SingleValue solution.
    #[serde(rename = "multi")]
    MultiValues,
}

/// Define how an int field should be handled by tantivy.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntOptions {
    indexed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    fast: Option<Cardinality>,
    stored: bool,
}

impl IntOptions {
    /// Returns true iff the value is stored.
    pub fn is_stored(&self) -> bool {
        self.stored
    }

    /// Returns true iff the value is indexed.
    pub fn is_indexed(&self) -> bool {
        self.indexed
    }

    /// Returns true iff the value is a fast field.
    pub fn is_fast(&self) -> bool {
        self.fast.is_some()
    }

    /// Set the u64 options as stored.
    ///
    /// Only the fields that are set as *stored* are
    /// persisted into the Tantivy's store.
    pub fn set_stored(mut self) -> IntOptions {
        self.stored = true;
        self
    }

    /// Set the u64 options as indexed.
    ///
    /// Setting an integer as indexed will generate
    /// a posting list for each value taken by the integer.
    pub fn set_indexed(mut self) -> IntOptions {
        self.indexed = true;
        self
    }

    /// Set the u64 options as a single-valued fast field.
    ///
    /// Fast fields are designed for random access.
    /// Access time are similar to a random lookup in an array.
    /// If more than one value is associated to a fast field, only the last one is
    /// kept.
    pub fn set_fast(mut self, cardinality: Cardinality) -> IntOptions {
        self.fast = Some(cardinality);
        self
    }

    /// Returns the cardinality of the fastfield.
    ///
    /// If the field has not been declared as a fastfield, then
    /// the method returns None.
    pub fn get_fastfield_cardinality(&self) -> Option<Cardinality> {
        self.fast
    }
}

impl Default for IntOptions {
    fn default() -> IntOptions {
        IntOptions {
            indexed: false,
            stored: false,
            fast: None,
        }
    }
}

impl From<()> for IntOptions {
    fn from(_: ()) -> IntOptions {
        IntOptions::default()
    }
}

impl From<FastFlag> for IntOptions {
    fn from(_: FastFlag) -> Self {
        IntOptions {
            indexed: false,
            stored: false,
            fast: Some(Cardinality::SingleValue),
        }
    }
}

impl From<StoredFlag> for IntOptions {
    fn from(_: StoredFlag) -> Self {
        IntOptions {
            indexed: false,
            stored: true,
            fast: None,
        }
    }
}

impl From<IndexedFlag> for IntOptions {
    fn from(_: IndexedFlag) -> Self {
        IntOptions {
            indexed: true,
            stored: false,
            fast: None,
        }
    }
}

impl<T: Into<IntOptions>> BitOr<T> for IntOptions {
    type Output = IntOptions;

    fn bitor(self, other: T) -> IntOptions {
        let mut res = IntOptions::default();
        let other = other.into();
        res.indexed = self.indexed | other.indexed;
        res.stored = self.stored | other.stored;
        res.fast = self.fast.or(other.fast);
        res
    }
}

impl<Head, Tail> From<SchemaFlagList<Head, Tail>> for IntOptions
where
    Head: Clone,
    Tail: Clone,
    Self: BitOr<Output = Self> + From<Head> + From<Tail>,
{
    fn from(head_tail: SchemaFlagList<Head, Tail>) -> Self {
        Self::from(head_tail.head) | Self::from(head_tail.tail)
    }
}
