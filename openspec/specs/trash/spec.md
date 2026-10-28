# trash Specification

## Purpose

Define what a trashed item is, how long it stays recoverable, and what the gallery must never do
while moving, restoring or permanently deleting it.

## Requirements

### Requirement: Trashed entry naming

The gallery SHALL store a trashed file under the trash folder using the name
`<unix-seconds>_<percent-encoded original path>`, where percent-encoding covers `/`, `%`, `\`, `:`,
every byte outside `0x20..=0x7E`, and every non-UTF-8 byte.

#### Scenario: Awkward original path round-trips

- GIVEN an original path containing slashes, a space, a literal `%`, a colon, non-ASCII text and an
  emoji
- WHEN the entry name is produced and then parsed
- THEN the parsed original path equals the input path exactly
- AND the produced name contains no `/` or `\` character

#### Scenario: Foreign name is not guessed

- GIVEN a file in the trash folder named `junk.txt` or `abc_x.jpg`
- WHEN its entry name is parsed
- THEN parsing fails
- AND the gallery does not invent an original location for it

### Requirement: Retention window

The gallery SHALL treat a trashed entry as expired only when more than 30 days (2,592,000 seconds)
have elapsed since the timestamp in its name.

#### Scenario: Exactly 30 days is kept

- GIVEN a trashed entry created exactly 30 days ago
- WHEN the expiry check runs
- THEN the entry is not expired and is not erased

#### Scenario: Older than 30 days is erased

- GIVEN a trashed entry created 30 days and one second ago
- WHEN the expiry check runs
- THEN the entry is expired and eligible for permanent removal

### Requirement: Restore never overwrites

When restoring, the gallery SHALL move the entry back to its decoded original path and, if anything
already exists there, SHALL place the restored copy next to it as `<stem> (restored)<ext>` and then
`<stem> (restored <n>)<ext>` for further collisions, leaving the existing file untouched.

#### Scenario: Existing file at the original path

- GIVEN a trashed entry whose original path is now occupied by a newer file
- WHEN the entry is restored
- THEN the newer file keeps its content
- AND the restored copy lands at a different path whose name contains ` (restored)`

#### Scenario: Second collision in the same restore

- GIVEN a restore attempt where both the original path and `<stem> (restored)<ext>` exist
- WHEN the entry is restored
- THEN the restored copy lands at `<stem> (restored 2)<ext>`

### Requirement: Permanent delete only inside the trash

The gallery SHALL refuse to permanently delete any path whose parent is not exactly the trash folder,
including paths that reach outside it through `..`.

#### Scenario: Path outside the trash is refused

- GIVEN a file outside the trash folder
- WHEN permanent deletion is requested for it
- THEN the operation returns an error
- AND the file still exists

#### Scenario: Traversal into the trash is refused

- GIVEN the path `<trash>/../precious.jpg`
- WHEN permanent deletion is requested
- THEN the operation returns an error
- AND the file outside the trash still exists

### Requirement: Moving into the trash cannot lose data

The gallery SHALL move a file into the trash with a rename when possible and otherwise by copying it,
verifying the copied length, and only then removing the source; a failed or incomplete copy SHALL
leave the source in place and produce no destination file.

#### Scenario: Cross-filesystem move with an incomplete copy

- GIVEN a rename that fails because source and destination are on different filesystems
- WHEN the copy produces fewer bytes than the source
- THEN the operation returns an error
- AND the destination file is removed
- AND the source file is still present

#### Scenario: Trashing an item twice is refused

- GIVEN a file that is already in the trash folder
- WHEN it is moved to the trash again
- THEN the operation returns an error
- AND no second entry is created
