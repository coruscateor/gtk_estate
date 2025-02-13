use crate::{WidgetUpgradeError, WidgetUpgradeResult};

pub fn on_widget_upgrade_error<F>(result: WidgetUpgradeResult, error_fn: F) -> bool
    where F: FnOnce(WidgetUpgradeError)
{

    if let Err(err) = result
    {

        error_fn(err);

        return true;

    }

    false

}

pub fn on_widget_upgrade_error_with_param<F, P>(result: WidgetUpgradeResult, error_fn: F, param: P) -> bool
    where F: FnOnce(WidgetUpgradeError, P)
{

    if let Err(err) = result
    {

        error_fn(err, param);

        return true;

    }

    false

}

pub fn widget_upgrade_error_display_println(result: WidgetUpgradeResult) -> bool
{

    on_widget_upgrade_error(result, |err|
    {

        println!("Widget upgrade error: {}\n\n", err);

    })

}

pub fn widget_upgrade_error_debug_println(result: WidgetUpgradeResult) -> bool
{

    on_widget_upgrade_error(result, |err|
    {

        println!("Widget upgrade error: {:?}\n\n", err);

    })

}

pub fn widget_upgrade_error_display_panic(result: WidgetUpgradeResult) -> bool
{

    on_widget_upgrade_error(result, |err|
    {

        panic!("Widget upgrade error: {}", err);

    })

}

pub fn widget_upgrade_error_debug_panic(result: WidgetUpgradeResult) -> bool
{

    on_widget_upgrade_error(result, |err|
    {

        panic!("Widget upgrade error: {:?}", err);

    })

}


