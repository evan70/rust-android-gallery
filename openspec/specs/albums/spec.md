# albums Specification

## Purpose

Define what an album is, how built-in albums differ from folders, how pins, counts and covers work,
and what renaming, deleting, moving and copying an album must do with the files on disk.

## Requirements

### Requirement: Album identity

The gallery SHALL treat a real album as a folder path, and SHALL use built-in album keys that start
with the control character `U+0001` (`all`, `videos`, `favs`, `trash`, `hidden`, `ext`) so that a
built-in key can never equal a folder path.

#### Scenario: Built-in keys cannot collide with folders

- GIVEN a real folder whose name is `all`
- WHEN the library is indexed
- THEN the built-in All album and the folder album are two distinct albums

#### Scenario: Persisted keys survive the reserved character

- GIVEN a pins file that stores a built-in key as `@all`
- WHEN the pins are read back
- THEN the key is restored to `U+0001all`

### Requirement: Album inventory with counts and covers

The gallery SHALL list every folder that contains current items, reporting for each album a count of
its current items and the newest item as its cover; trashed, hidden and permanently deleted items
SHALL NOT be counted.

#### Scenario: Count and cover reflect current items

- GIVEN an album with three current items where the newest is a 2024 photo
- WHEN the Albums view is built
- THEN the album count is 3
- AND its cover is that 2024 photo
- AND an item moved to the trash no longer contributes to the count

### Requirement: Default pins on a fresh install

On a fresh install the gallery SHALL pin All, the largest folder named `Camera`, the largest folder
named `Screenshots`, Videos and Favourites, in that order, skipping the folder pins that do not exist.

#### Scenario: Camera folder is pinned by size

- GIVEN two folders named `Camera` with 4 and 11 items and no stored pins
- WHEN the default pins are built
- THEN the pinned Camera album is the one with 11 items

#### Scenario: Missing folders are skipped

- GIVEN a library without a `Screenshots` folder
- WHEN the default pins are built
- THEN All, Videos and Favourites are pinned
- AND no empty Screenshots pin exists

### Requirement: Album names are made safe

The gallery SHALL convert a typed album name into a folder name by trimming whitespace, replacing
`/`, `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|` and control characters with `_`, stripping leading dots,
limiting the result to 60 characters, and rejecting it when nothing usable remains.

#### Scenario: Path separators are neutralised

- GIVEN the typed name `Holiday/2024: best?`
- WHEN it is converted to a folder name
- THEN the result contains no `/` or `:` or `?` character

#### Scenario: Unusable name is rejected

- GIVEN the typed name `...`
- WHEN it is converted to a folder name
- THEN no folder name is produced and no album is created

### Requirement: Moving and copying never overwrite

The gallery SHALL move an item into an album only when it is not already in that album, SHALL place
it under `<name> (1).ext`, `<name> (2).ext` … when the name is taken, SHALL verify the copied byte
count before treating a copy as complete, and SHALL keep the original modification time on a copy.

#### Scenario: Existing file is not overwritten

- GIVEN an album that already contains `IMG_1.jpg`
- WHEN another `IMG_1.jpg` is moved into it
- THEN the existing file keeps its content
- AND the moved file lands as `IMG_1 (1).jpg`

#### Scenario: Incomplete copy is discarded

- GIVEN a copy that produces fewer bytes than the source
- WHEN the copy finishes
- THEN the partial destination is removed
- AND the operation reports an error

#### Scenario: Copy keeps the timeline position

- GIVEN a photo modified on 3 March
- WHEN it is copied into another album
- THEN the copy keeps the 3 March modification time

### Requirement: Renaming an album follows the folder

The gallery SHALL rename the album folder and update the stored item paths, album keys, favourites
and pins that pointed inside it; it SHALL refuse the rename when the target name already exists.

#### Scenario: Items and pins follow the rename

- GIVEN an album containing one item and a pin on that album
- WHEN the album is renamed
- THEN the item's path points into the new folder
- AND the pin points at the new folder
- AND the item is not lost from the view

#### Scenario: Existing target is refused

- GIVEN an album name that already exists on disk
- WHEN the rename is requested
- THEN no folder is renamed and the user is told the name is taken

### Requirement: Deleting an album trashes its contents

The gallery SHALL move every current item of a deleted album to the trash, remove the folder only
when it ends up empty, drop the album's pin, and close the album view when it was open.

#### Scenario: Items go to the trash, pin disappears

- GIVEN an album with two items and a pin on it
- WHEN the album is deleted
- THEN both items are in the trash
- AND the pin no longer exists
- AND the folder is removed because it is empty

#### Scenario: Non-empty folder is kept

- GIVEN an album folder that still holds a file the gallery does not manage
- WHEN the album is deleted
- THEN the managed items are trashed
- AND the folder and the unmanaged file remain
