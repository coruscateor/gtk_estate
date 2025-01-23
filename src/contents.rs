
///
/// Using a Box to frame your content? Us this macro to access it.
/// 
/// gtk4::Box needs to be in scope. 
/// 
#[macro_export]
macro_rules! impl_contents_box_ref
{

    () =>
    {

        fn contents_box_ref(&self) -> &Box
        {

            &self.contents_box
            
        }

    };
    ($field_name:ident) =>
    {

        fn contents_box_ref(&self) -> &Box
        {

            &self.$field_name
            
        }

    }

}

