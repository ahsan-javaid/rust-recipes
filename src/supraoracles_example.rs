mod external{
    use core::marker::PhantomData;
    use core::fmt::Debug;

    pub struct Stream<T: Send + 'static + Debug>{
        _pd: PhantomData<T>
    }
    
    impl <T: Send + 'static + Debug> Stream<T>{
        /// returns the next element of the stream
        pub async fn next()-> T {
            unimplemented!();
        }
    }
    
    /// performs a call to a third party API
    pub async fn api_call(query_param: &str) -> String{
        unimplemented!();
    }
}

pub fn do_stuff_here(stream: external::Stream<String>){
    
}
