### Certs

This is where your public and private key sits, these are ofcourse ignored in git, and in the case of the private key, should never be shared, and should be stored somewhere securely. The ssl tool generates this key pair for local auth, and is ideal for the use of local development. For production, these keys should be in a very secure location and contain functionality to be rolled.
