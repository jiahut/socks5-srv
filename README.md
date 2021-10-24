a tiny socks5 proxy server

##  install 

> cargo build 

> cargo install -path . 

## usage

> socks5-srv # default port `1080`

> socks5-srv 10800 # or spectify port `10800`

## verify 

> curl -x socks5://127.0.0.1:1080 ip.sb
