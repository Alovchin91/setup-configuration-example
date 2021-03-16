# setup-configuration-sample

### Test cases

_Prerequisites for 2 and 3:_ Apply **register.reg** from the Gist: https://gist.github.com/heaths/d91d569a61151a7c9deb9c7beb8fc2fa


1. Run the sample (`cargo run`).

    _Expected output:_ `Ok(ISetupConfiguration { inner: ... })`

2. Uncomment line 26 (`const CLSCTX_LOCAL_SERVER`) and change `dwClsContext` parameter on line 42 to `CLSCTX_LOCAL_SERVER`. Run the sample.

    _Expected output:_ `Err(80004002)`
    
3. Change turbo-fish generic parameter to `IUnknown` on line 31 (was `ISetupConfiguration`). Run the sample.

    _Expected output:_ `Ok(IUnknown { inner: ... })`
