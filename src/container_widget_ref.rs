use std::any::Any;

use gtk::glib::{object::ObjectExt, types::StaticType};

use crate::gtk4 as gtk;

//Welp this is rudundant...
pub enum ContainerWidgetRef<'a>
{

    //GTK

    //GtkApplication(gtk::Application),
    GtkApplicationWindow(&'a gtk::ApplicationWindow),
    GtkAspectFrame(&'a gtk::AspectFrame),
    GtkBox(&'a gtk::Box),
    GtkCenterBox(&'a gtk::CenterBox),
    GtkExpander(&'a gtk::Expander),
    GtkFixed(&'a gtk::Fixed),
    GtkFlowBox(&'a gtk::FlowBox),
    GtkFlowBoxChild(&'a gtk::FlowBoxChild),
    GtkGrid(&'a gtk::Grid),
    //GtkGridLayout(&'a gtk::GridLayout),
    //GtkGridLayoutChild(&'a gtk::GridLayoutChild),
    GtkGridView(&'a gtk::GridView),
    GtkListBox(&'a gtk::ListBox),       //NEW
    GtkListBoxRow(&'a gtk::ListBoxRow), //NEW
    GtkNotebook(&'a gtk::Notebook),
    GtkNotebookPage(&'a gtk4::NotebookPage), //NEW
    GtkOverlay(&'a gtk::Overlay),
    GtkPaned(&'a gtk::Paned), //NEW
    GtkRevealer(&'a gtk::Revealer),
    GtkScrolledWindow(&'a gtk::ScrolledWindow),
    GtkSearchBar(&'a gtk::SearchBar),
    GtkStack(&'a gtk::Stack),
    GtkViewport(&'a gtk::Viewport),
    GtkWidgetPaintable(&'a gtk::WidgetPaintable),
    GtkWindow(&'a gtk::Window),

    //WidgetPaintable - https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.WidgetPaintable.html

    //ADW

    //cfg_if doesn't work here :(
    
    #[cfg(feature="adw")]
    AdwActionRow(&'a adw::ActionRow),
    #[cfg(feature="adw")]
    AdwApplication(&'a adw::Application),
    #[cfg(feature="adw")]
    AdwApplicationWindow(&'a adw::ApplicationWindow),
    #[cfg(feature="adw")]
    AdwBin(&'a adw::Bin),
    #[cfg(feature="adw")]
    AdwCarousel(&'a adw::Carousel),
    #[cfg(feature="adw")]
    AdwClamp(&'a adw::Clamp),
    #[cfg(feature="adw")]
    AdwClampScrollable(&'a adw::ClampScrollable),
    #[cfg(feature="adw")]
    AdwComboRow(&'a adw::ComboRow), //NEW
    /* Deprecated
    #[cfg(feature="adw")]
    AdwFlap(&'a adw::Flap),
    #[cfg(feature="adw")]
    AdwLeaflet(&'a adw::Leaflet),
    #[cfg(feature="adw")]
    AdwLeafletPage(&'a adw::LeafletPage),
    */
    #[cfg(feature="adw")]
    AdwPreferencesGroup(&'a adw::PreferencesGroup),
    #[cfg(feature="adw")]
    AdwPreferencesWindow(&'a adw::PreferencesWindow),
    /* Deprecated
    #[cfg(feature="adw")]
    AdwSqueezer(&'a adw::Squeezer),
    #[cfg(feature="adw")]
    AdwSqueezerPage(&'a adw::SqueezerPage),
    */
    #[cfg(feature="adw")]
    AdwStatusPage(&'a adw::StatusPage),
    #[cfg(feature="adw")]
    AdwTabPage(&'a adw::TabPage),
    #[cfg(feature="adw")]
    AdwTabView(&'a adw::TabView),
    #[cfg(feature="adw")]
    AdwToastOverlay(&'a adw::ToastOverlay), //NEW
    #[cfg(feature="adw")]
    AdwViewStack(&'a adw::ViewStack),
    #[cfg(feature="adw")]
    AdwViewStackPage(&'a adw::ViewStackPage),
    #[cfg(feature="adw")]
    AdwViewSwitcher(&'a adw::ViewSwitcher),
    #[cfg(feature="adw")]
    AdwWindow(&'a adw::Window)

}

impl<'a> ContainerWidgetRef<'a>
{

    pub fn new<T>(container_widget: &T) -> Result<ContainerWidgetRef, &'static str>
        where T: ObjectExt //+ Clone
    {

        let cwt = container_widget.type_();

        let any_container_widget: &dyn Any = container_widget;

        let unexpected_error_message = "Error: Unexpected cast error.";

        //GTK

        if gtk::ApplicationWindow::static_type() == cwt
        {

            let wref = any_container_widget.downcast_ref::<gtk::ApplicationWindow>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkApplicationWindow(wref));

        }
        else if gtk::AspectFrame::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::AspectFrame>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkAspectFrame(wref));

        }
        else if gtk::Box::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Box>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkBox(wref));

        }
        else if gtk::CenterBox::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::CenterBox>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkCenterBox(wref));

        }
        else if gtk::Expander::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Expander>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkExpander(wref));

        }
        else if gtk::Fixed::static_type() == cwt
        {

            let wref = any_container_widget.downcast_ref::<gtk::Fixed>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkFixed(wref));

        }
        else if gtk::FlowBox::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::FlowBox>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkFlowBox(wref));

        }
        else if gtk::FlowBoxChild::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::FlowBoxChild>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkFlowBoxChild(wref));

        }
        else if gtk::Grid::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Grid>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkGrid(wref));

        }
        else if gtk::Grid::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Grid>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkGrid(wref));

        }
        else if gtk::GridView::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::GridView>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkGridView(wref));

        }
        else if gtk::ListBox::static_type() == cwt
        {

            let wref = any_container_widget.downcast_ref::<gtk::ListBox>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkListBox(wref));

        }
        else if gtk::ListBoxRow::static_type() == cwt
        {

            let wref = any_container_widget.downcast_ref::<gtk::ListBoxRow>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkListBoxRow(wref));

        }
        else if gtk::Notebook::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Notebook>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkNotebook(wref));

        }
        else if gtk::NotebookPage::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::NotebookPage>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkNotebookPage(wref));

        }
        else if gtk::Overlay::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Overlay>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkOverlay(wref));

        }
        else if gtk::Revealer::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Revealer>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkRevealer(wref));

        }
        else if gtk::ScrolledWindow::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::ScrolledWindow>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkScrolledWindow(wref));

        }
        else if gtk::SearchBar::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::SearchBar>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkSearchBar(wref));

        }
        else if gtk::Stack::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Stack>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkStack(wref));

        }
        else if gtk::Viewport::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Viewport>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkViewport(wref));

        }
        else if gtk::WidgetPaintable::static_type() == cwt
        {

            let wref = any_container_widget.downcast_ref::<gtk::WidgetPaintable>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkWidgetPaintable(wref));

        }
        else if gtk::Window::static_type() == cwt
        {
            
            let wref = any_container_widget.downcast_ref::<gtk::Window>().expect(unexpected_error_message);

            return Ok(ContainerWidgetRef::GtkWindow(wref));

        }

        //ADW

        //expected expression, found keyword `else`

        cfg_if::cfg_if! 
        {
        
            if #[cfg(feature="adw")]
            {

                if adw::ActionRow::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ActionRow>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwActionRow(wref));

                }
                else if adw::ActionRow::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ActionRow>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwActionRow(wref));

                }
                else if adw::Application::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::Application>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwApplication(wref));

                }
                else if adw::ApplicationWindow::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ApplicationWindow>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwApplicationWindow(wref));

                }
                else if adw::Bin::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::Bin>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwBin(wref));

                }
                else if adw::Carousel::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::Carousel>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwCarousel(wref));

                }
                else if adw::Clamp::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::Clamp>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwClamp(wref));

                }
                else if adw::ClampScrollable::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ClampScrollable>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwClampScrollable(wref));

                }
                else if adw::ComboRow::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ComboRow>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwComboRow(wref));

                }
                /*
                else if adw::Flap::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::Flap>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwFlap(wref));

                }
                else if adw::Leaflet::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::Flap>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwFlap(wref));

                }
                */
                //leaflet_pages: RefCell<Map<adw::LeafletPage>>,
                else if adw::PreferencesGroup::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::PreferencesGroup>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwPreferencesGroup(wref));

                }
                else if adw::PreferencesWindow::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::PreferencesWindow>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwPreferencesWindow(wref));

                }
                //adw::Squeezer
                //adw::SqueezerPage
                else if adw::StatusPage::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::StatusPage>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwStatusPage(wref));

                }
                else if adw::TabPage::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::TabPage>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwTabPage(wref));

                }
                else if adw::TabView::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::TabView>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwTabView(wref));

                }
                else if adw::ToastOverlay::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ToastOverlay>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwToastOverlay(wref));

                }
                else if adw::ViewStack::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ViewStack>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwViewStack(wref));

                }
                else if adw::ViewStackPage::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ViewStackPage>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwViewStackPage(wref));

                }
                else if adw::ViewSwitcher::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::ViewSwitcher>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwViewSwitcher(wref));

                }
                else if adw::Window::static_type() == cwt
                {

                    let wref = any_container_widget.downcast_ref::<adw::Window>().expect(unexpected_error_message);

                    return Ok(ContainerWidgetRef::AdwWindow(wref));

                }
                    
            }

        }
        Err("Error: Widget of invalid type provided.")

        /*
        match cwt.name()
        {

            gtk::ApplicationWindow::static_type().name() => {

                

            }
            
        }
        */

    }

}