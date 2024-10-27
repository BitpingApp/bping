# bping

![Bping demo](./bping-demo.gif)

````
A command line utility to ping a website from anywhere in the world!

Usage: bping [-r=<regions>] [-c=<count>] [-a=<attempts>] --api-key=<api_key> <endpoint>

Available positional items:
    <endpoint>               Specifies the endpoint (without http://) to ping. eg. bitping.com

Available options:
    -r, --regions=<regions>  Specifies the ISO 3166-1 country codes (alpha-2 or alpha-2) & continent
                             names to send jobs to. Defaults to Anywhere.
                                     (eg. bping -r "AU,CHN,North America" bitping.com)
    -c, --count=<count>      Specifies the number of ICMP packets to send per country. Defaults to
                             3.
    -a, --attempts=<attempts>  Specifies the number of ping attempts per country. Defaults to 1.
        --api-key=<api_key>  Specifies the API key for authentication. Can also be set using the
                             BITPING_API_KEY environment variable.
    -h, --help               Prints help information
    -V, --version            Prints version information****```
````
