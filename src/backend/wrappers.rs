#[macro_export]
macro_rules! create_vec_wrapper_simple {
    ( $name:tt, $inner_type:ty ) => {
        #[derive(Clone, PartialEq, Debug, Default)]
        pub struct $name (pub Vec<$inner_type>);
    };
}

#[macro_export]
macro_rules! impl_deref_for_vec_wrapper {
    ( $wrapper:ty, $inner_type:ty ) => {
        impl std::ops::Deref for $wrapper {
            type Target = Vec<$inner_type>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_derefmut_for_vec_wrapper {
    ( $wrapper:ty, $inner_type:ty ) => {
        impl std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_for_vec_wrapper {
    ( $wrapper:ty, $inner_type:ty ) => {
        impl From<Vec<$inner_type>> for $wrapper {
            fn from(value: Vec<$inner_type>) -> Self {
                Self(value)
            }
        } 
    };
}

#[macro_export]
macro_rules! impl_into_for_vec_wrapper {
    ( $wrapper:ty, $inner_type:ty ) => {
        impl Into<Vec<$inner_type>> for $wrapper {
            fn into(self) -> Vec<$inner_type> {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_as_ref_for_vec_wrapper {
    ( $wrapper:ty, $inner_type:ty ) => {
        impl AsRef<Vec<$inner_type>> for $wrapper {
            fn as_ref(&self) -> &Vec<$inner_type> {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_as_mut_for_vec_wrapper {
    ( $wrapper:ty, $inner_type:ty ) => {
        impl AsMut<Vec<$inner_type>> for $wrapper {
            fn as_mut(&mut self) -> &mut Vec<$inner_type> {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_new_for_vec_wrapper {
    ( $wrapper:ty) => {
        impl $wrapper {
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
}


#[macro_export]
macro_rules! create_vec_wrapper {
    ( $name:tt, $inner_type:ty) => {
        $crate::create_vec_wrapper_simple!($name, $inner_type);

        $crate::impl_deref_for_vec_wrapper!($name, $inner_type);
        $crate::impl_derefmut_for_vec_wrapper!($name, $inner_type);
        $crate::impl_from_for_vec_wrapper!($name, $inner_type);
        $crate::impl_into_for_vec_wrapper!($name, $inner_type);

        $crate::impl_as_ref_for_vec_wrapper!($name, $inner_type);
        $crate::impl_as_mut_for_vec_wrapper!($name, $inner_type);

        $crate::impl_new_for_vec_wrapper!($name);
    };
    ( $name:tt, $inner_type:ty, GetEWs) => {
        // $crate::create_vec_wrapper_simple!($name, $inner_type);

        // $crate::impl_deref_for_vec_wrapper!($name, $inner_type);
        // $crate::impl_derefmut_for_vec_wrapper!($name, $inner_type);
        // $crate::impl_from_for_vec_wrapper!($name, $inner_type);
        // $crate::impl_into_for_vec_wrapper!($name, $inner_type);
        $crate::create_vec_wrapper!($name, $inner_type);

        $crate::impl_getews_for_vec_wrapper!($name, $inner_type);
    };
}






// macro_rules! create_vec_wrapper {
//     ( $name:tt, $inner_type:ty) => {
        // #[derive(Clone, PartialEq, Debug, Default)]
        // pub struct $name (pub Vec<$inner_type>);

        // impl std::ops::Deref for $name {
        //     type Target = Vec<$inner_type>;
        //     fn deref(&self) -> &Self::Target {
        //         &self.0
        //     }
        // }

        // impl std::ops::DerefMut for $name {
        //     fn deref_mut(&mut self) -> &mut Self::Target {
        //         &mut self.0
        //     }
        // }

        // impl From<Vec<$inner_type>> for $name {
        //     fn from(value: Vec<$inner_type>) -> Self {
        //         Self(value)
        //     }
        // } 

        // impl Into<Vec<$inner_type>> for $name {
        //     fn into(self) -> Vec<$inner_type> {
        //         self.0
        //     }
        // }
//     };
// }