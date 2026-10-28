# Delta for Vault

## Purpose

Define where the hidden album lives, what makes it invisible to other apps, what the lock must
guarantee, and how hidden items return to where they came from.

## ADDED Requirements

### Requirement: Vault location and marker

The gallery SHALL keep hidden items in a dot-folder (`/sdcard/.GalleryHidden` on Android, a temp
folder in the desktop preview) that contains an empty `.nomedia` file, and SHALL ensure the folder
and marker exist before the first item is moved into it.

#### Scenario: First hide creates the folder and marker

- GIVEN a vault folder that does not exist yet
- WHEN the first item is hidden
- THEN the folder is created
- AND an empty `.nomedia` file exists inside it
- AND the item is present in the vault folder

#### Scenario: Marker write failure does not fail the hide

- GIVEN a vault folder where writing `.nomedia` fails
- WHEN an item is hidden
- THEN the file is still moved and the item still reports as hidden
- AND no error is surfaced to the user for the marker alone

### Requirement: Unlock required before the vault is shown

The gallery SHALL NOT list, render thumbnails of, or open the hidden album until the phone's
fingerprint or screen lock has confirmed the unlock; when no screen lock is configured it SHALL tell
the user to set one instead of opening the vault.

#### Scenario: Locked vault stays hidden

- GIVEN the vault is locked
- WHEN the user opens the Hidden album
- THEN the system lock prompt is requested
- AND hidden items are not listed and no thumbnail is decoded for them

#### Scenario: Unlock without a configured screen lock

- GIVEN a device with no screen lock
- WHEN the user opens the Hidden album
- THEN the vault does not open
- AND the user is told to set a screen lock in Android settings

### Requirement: Hiding preserves the original location

The gallery SHALL record where each hidden item came from and SHALL return it there on unhide, or to
a restored-name sibling when that path is occupied.

#### Scenario: Hide then unhide returns to the original folder

- GIVEN an item in `DCIM/Camera`
- WHEN it is hidden and then unhidden
- THEN it is back in `DCIM/Camera`
- AND it is no longer marked as hidden

#### Scenario: Unhide when the original path is occupied

- GIVEN a hidden item whose original path now holds another file
- WHEN it is unhidden
- THEN the other file is untouched
- AND the unhidden copy lands at `<stem> (restored)<ext>`

### Requirement: Locking purges cached pixels

The gallery SHALL drop the cached thumbnails and video durations of vault items when the vault is
locked, and SHALL leave thumbnails of non-vault items untouched.

#### Scenario: Lock clears only vault thumbnails

- GIVEN an unlocked vault with cached thumbnails for two hidden items and one visible item
- WHEN the vault is locked
- THEN the two hidden thumbnails and durations are no longer held in memory
- AND the visible item keeps its cached thumbnail

### Requirement: Vault moves share the trash naming scheme

The gallery SHALL name files inside the vault with the same
`<unix-seconds>_<percent-encoded original path>` scheme used by the trash, so the original location
can be decoded after a restart.

#### Scenario: Original path survives an app restart

- GIVEN an item hidden in an earlier session
- WHEN the library is scanned again
- THEN the item is classified as hidden
- AND its original location is still decodable from its file name
