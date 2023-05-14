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
# init db in default dir
mypeople init

# init db specific dir
mypeople init path
```

### add command
```sh
# add contact
mypeople add contact_name

# add contacts fields to a existing contact
mypeople add contact_name -e "email_label:email_value"

# add contacts with fields
mypeople add new_contact_name -p "phone_label:phone_number" -w "wallet_label:wallet_address"
```

### ls command
```sh
# list all contacts with all his fields
mypeople ls

# list subset of contacts with all his fields
mypeople ls contact_name contact_name2

# list contact specific fields
mypeople ls contact_name -g -pe

# list specific fields of a subset of contacts 
mypeople ls contact_name contact_name2 -sw
```

### rm command
```sh
# remove specific contacts
mypeople rm contact_name1 contact_name2

# remove specific field from contact existing contact `contact_name`
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
# create config file
mkdir -p $HOME/.config/mypeople/mypeople.toml
```

```sh
# create cache directory for store db
mkdir -p $HOME/.cache/mypeople
```

# Documentation



# Troubleshooting



# License
