#[macro_export]
macro_rules! route {
    (pub async fn $fn_name:ident [ $state_var:ident : $state_name:ident / $state_const:ident ] ( $( $variable:ident : $typ:ty ),* ) -> $ret:ty = if $enabled:block else $disabled:block ) => {
        pub static $state_const: $state_name = $state_name {
            enabled: ::std::sync::atomic::AtomicBool::new(true),
            hits: ::std::sync::atomic::AtomicUsize::new(0),
        };

        pub struct $state_name {
            pub enabled: ::std::sync::atomic::AtomicBool,
            pub hits: ::std::sync::atomic::AtomicUsize,
        }

        pub async fn $fn_name( $( $variable : $typ ),* ) -> $ret {
            let $state_var: &$state_name = &$state_const;

            if $state_var.enabled.load(::std::sync::atomic::Ordering::SeqCst) {
                let _ = $state_var.hits.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst);

                $enabled
            } else {
                $disabled
            }
        }
    };
}
