# Donc
 
Clash of Clan players, have you ever asked the following question: What can my clan give me ?
Donc is the solution. It takes a clan tag (*ex : #22Q992VQG*) a will give you a nice [CSV](https://en.wikipedia.org/wiki/Comma-separated_values) file with all the troops and spells that your clan mates can give you.

![table](https://raw.githubusercontent.com/tatounee/Donc/main/screenshot/table.png)

## Setup

### Downloading
- **Windows**
You can find the latest version of Donc in the [release tab](https://github.com/Tatounee/Donc/releases/tag/v.1.0).

- **Linux and macOS**
You will need to compile yourself Donc with rust.

### Clash of Clans key
You will need an API key to use Donc. You can create one on the [official supercell website](https://developer.clashofclans.com/#/).
If you already have an account, create a new key and copy its TOKEN. If not, create one, then create a new key and copy its TOKEN.

![token](https://raw.githubusercontent.com/tatounee/Donc/main/screenshot/token.png)

Finally, create a `.env` file at the root of your disk and paste in this code `COC_KEY_TOKEN = <token>` replacing `<token>` with your token

![env](https://raw.githubusercontent.com/tatounee/Donc/main/screenshot/env.png)

### Execute
Open a command prompt and execute in `donc.exe` with the clan tag that you want to know these donations.
Example : `$ donc PV8QR8VO`

![example](https://raw.githubusercontent.com/tatounee/Donc/main/screenshot/example.png)

After that, you will end up with a beautiful CSV file full of all data you want.

---

*This material is unofficial and is not endorsed by Supercell. For more information see [Supercell's Fan Content Policy](https://www.supercell.com/fan-content-policy)*