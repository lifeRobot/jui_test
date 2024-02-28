use std::time::Duration;
use cbsk_base::once_cell::sync::Lazy;
use cbsk_base::tokio;
use cbsk_mut_data::mut_data_obj::MutDataObj;

#[allow(non_upper_case_globals)]
pub static app: Lazy<MutDataObj<App>> = Lazy::new(|| { MutDataObj::new(App::new().unwrap()) });

#[allow(non_upper_case_globals)]
pub static global_bool: Lazy<MutDataObj<bool>> = Lazy::new(MutDataObj::default);

slint::include_modules!();

#[tokio::main]
async fn main() {
    cbsk_run::async_pool::push(async {
        let mut test_data = TestData{
            confirm_data: "hello".into(),
            hello_world: "hello data".into(),
            index: 1,
        };

        loop {
            if global_bool.get() {
                test_data.hello_world = "hello".into();
                test_data.confirm_data = "hello data".into();
            } else {
                test_data.hello_world = "world".into();
                test_data.confirm_data = "world data".into();
            }

            app.global::<TestGlobal>().set_test(test_data.clone());
            global_bool.trigger();

            // one sec change TestGlobal once
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // start window
    app.run().unwrap();

    // wait global async
    slint::spawn_local(async { cbsk_run::async_pool::listener().await.unwrap(); }).unwrap();
}
