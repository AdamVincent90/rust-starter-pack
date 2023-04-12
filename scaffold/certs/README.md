### Certs

This is where your public and private key sits, these are ofcourse ignored in git, and in the case of the private key, should never be shared, and should be stored somewhere securely. The ssl tool generates this key pair for local auth, and is ideal for the use of local development. For production, these keys should be in a very secure location and contain functionality to be rolled.

Also, the private key on generation will contain a unique uuid as the file name. This allows for multiple keys for different auth if required,
This unique UUID should then be stored with your env file as way for your application to identify the correct key.

Reference, to generate a unique RSA256 keypair

`make rsa-keypair`
