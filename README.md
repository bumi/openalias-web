# OpenAlias Web

This tool creates custom pages with crypto-currency payment details for any domain with [OpenAlias](https://openalias.org) entries.  
It can be used as a tipping/donation page. No setup or signup is required. All details are gathered exlusively from the OpenAlias DNS entries.


## What is OpenAlias? 

OpenAlias ([openalias.org](https://openalias.org)) is an open standard for simpler addresses for any crypto currencies. [Read more here](https://openalias.org)

    At its most basic, OpenAlias is a TXT DNS record on a FQDN (fully qualified domain name). 
    By combining this with DNS-related technologies [it has] created an aliasing standard that is extensible for developers, 
    intuitive and familiar for users, and can interoperate with both centralised and decentralised domain systems.


## Example: 

Here is the page for the entries from `donate@getmonero.org`:

[[openalias-web-host]/donate@getmonero.org](https://openalias-web.herokuapp.com/donate@getmonero.org)


## How to get your page

If you already have a domain with OpenAlias DNS entries you already have one: `https://openalias-web.herokuapp.com/YOUR DOMAIN`  
If not, simply configure your DNS entries as described on [OpenAlias.org](https://openalias.org) (because of DNS propagation it might take a bit until your page is available)


## Development 

The app is written in Rust and uses the [rocket.rs](https://rocket.rs/) web framework. 

To run the app locally: 

    $ cargo run
    
    
Build a release:

    $ cargo build --release
 

### Deployment

The app is deployed on Heroku using the buildpack: [emk/heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust).


## ToDo

- DESIGN, DESIGN, DESIGN - a properly design page is required... can you help?
- get a proper short domain

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/bumi/openalias-web .  
(this is my first Rust code, feedback and fixes are very welcome :) 

## License

The tool is as available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
