# Index

1. [Purpose](#Purpose)
2. [Usage](#Usage)
3. [Installation](#Installation)
4. [Configuration](#Configuration)
5. [Troubleshooting](#Troubleshooting)
6. [License](#License)

# Purpose

**mypeople purpose is to help you manage your contacts**.Apps that do
this are commonly called address books. I personally
prefer contact book because i find the last more intuitive.

# Usage

```sh
# init db
mypeople init   
mypeople init path   
```

```sh
# add contact
mypeople add contact_name

# add contacts fields to existing contact
mypeople add contact_name -e "email_label:email_number"

# add contacts with fields
mypeople add contact_name2 -p "phone_label:phone_number" -w "wallet_label:wallet_address"
```

```sh
# list all contacts
mypeople ls

# list subset of contacts 
mypeople ls contact_name contact_name2
```

```sh
# remove contacts
mypeople rm contact_name

# remove contacts field
mypeople rm contact_name -e email_label
```

# Installation 

### From source
```sh
git clone github.com/TheSnakeWitcher/mypeople
make release
```

# Configuration


# Documentation


# Troubleshooting


# License
