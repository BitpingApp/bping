# bping

![Bping demo](./bping-demo.gif)

## 🚀 Ready to Start Pinging?

1. Head over to [developer.bitping.com](https://developer.bitping.com/pricing) to create your free account
2. Create your API key in the developer dashboard
3. Set the API Key environment variable
   - eg. `export BITPING_API_KEY=your_api_key`

With your API key in hand, you'll be ready to ping from anywhere in the world!

## Linux/MacOS Installation

You can install bping directly using this command:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/BitpingApp/bping/releases/latest/download/bping-installer.sh | sh
```

#### Install prebuilt binaries via Homebrew

```sh
brew install BitpingApp/tap/bping
```

## Windows Installation

You can install bping on Windows by running this command in PowerShell:

```sh
powershell -ExecutionPolicy ByPass -c "irm https://github.com/BitpingApp/bping/releases/latest/download/bping-installer.ps1 | iex"
```

#### Install using Windows Installer

Go to the latest release and download the .msi for windows.


### Help Documentation

---

````

A command line utility to ping a website from anywhere in the world!

Usage: bping [-r=<regions>] [-c=<count>] [-a=<attempts>] --api-key=<api_key> <endpoint>

Available positional items:
<endpoint> Specifies the endpoint (without http://) to ping. eg. bitping.com

Available options:
-r, --regions=<regions> Specifies the ISO 3166-1 country codes (alpha-2 or alpha-2) & continent
names to send jobs to. Defaults to Anywhere.
(eg. bping -r "AU,CHN,North America" bitping.com)
-c, --count=<count> Specifies the number of ICMP packets to send per country. Defaults to 3.
-a, --attempts=<attempts> Specifies the number of ping attempts per country. Defaults to 1.
--api-key=<api_key> Specifies the API key for authentication. Can also be set using the
BITPING_API_KEY environment variable.
-h, --help Prints help information
-V, --version Prints version information\*\*\*\*```

````

```

```
