# Index


1. [Purpose](#Purpose)
2. [Usage](#Usage)
3. [Installation](#Installation)
4. [Configuration](#Configuration)
5. [Documentation](#Documentation)
6. [Troubleshooting](#Troubleshooting)
7. [License](#License)


# Purpose


**mypeople purpose is to help you manage your contacts**. Apps that do
this are commonly called address books. I personally prefer contact
book because i find the last more intuitive.


# Usage


### init command

```sh
# init db in default dir $HOME/.cache/mypeople/mypeople.db
mypeople init

# init db in path
mypeople init path
```

### add command

```sh
# add a new contact
mypeople add contact_name

# add fields to an existing contact
mypeople add contact_name -e "email_label:email_value" -s "social_media_label:social_media_value" 

# add a new contacts with fields
mypeople add new_contact_name -p "phone_label:phone_number" -w "wallet_label:wallet_address"
```

### ls command

```sh
# list all contacts with all his fields
mypeople ls

# list specific fields of all contacts
mypeople ls -espw

# list subset of contacts with all his fields
mypeople ls contact_name contact_name2

# list specific fields of a subset of contacts 
mypeople ls contact_name contact_name2 -es
```

### rm command

```sh
# remove specific contacts
mypeople rm contact_name1 contact_name2

# remove specific field from existing contact
mypeople rm contact_name -e email_label -p phone_label
```


# Installation 


### From source

```sh
git clone github.com/TheSnakeWitcher/mypeople
make release
```


# Configuration


```sh
mkdir -p $HOME/.config/mypeople/mypeople.toml  # create config file
mkdir -p $HOME/.cache/mypeople                 # create cache directory for store db
```

Config file `mypeople.toml` only support `dbfile`
which value is a path to a selected database created
with `mypeople init`.


# Documentation




# Troubleshooting




# License


MIT
