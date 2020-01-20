
#[macro_export]   
/// From is not that usefull

macro_rules! mimpl {
    (Derive; $( $derivable:expr ),+ ) => {
        #[derive( $($derivable),+) ]
    };
    (From; $from:path, $type:path, $closure:expr) => {
        impl From<$from> for $type {
            fn from(from: $from) -> Self {
                $closure(from)
            }
        }
    };
    (TryFrom; $from:path, $type:path, $error:ty, $closure:expr) => {
        impl TryFrom<$from> for $type {
            type Error = $error;
            fn try_from(from: $from) -> Result<Self, Self::Error> {
                $closure(from)
            }
        }
    };
    
    (Ord; $type:path, $closure:expr) => {
        impl Ord for $type {
            fn cmp(&self, other: &Self) -> Ordering {
                $closure(self, other)
            }
        }
    };
  
    (PartialOrd; $type:path, USE_CMP) => {
        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
    };
    (PartialOrd; $type:path, $closure:expr) => {
        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                $closure(self, other)
            }
        }
    };
}
