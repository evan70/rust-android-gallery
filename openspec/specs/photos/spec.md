# photos Specification

## Purpose

Define which files the gallery treats as media, how it walks the device's storage, in what order they
are shown, and how the date-grouped grid is laid out.

## Requirements

### Requirement: Media discovery

The gallery SHALL discover images with the extensions `jpg`, `jpeg`, `png`, `webp`, `gif`, `bmp` and
videos with `mp4`, `mkv`, `webm`, `3gp`, `3g2`, `mov`, `m4v`, `ts`, `m2ts`, `mts`, `avi`, `mpg`,
`mpeg`, `wmv`, `flv`, `ogv`, matching the extension case-insensitively.

#### Scenario: Both media families are discovered

- GIVEN a folder containing `IMG_0001.JPG`, `clip.MP4` and `notes.txt`
- WHEN the folder is scanned
- THEN two items are found
- AND the `.JPG` file is not treated as a video
- AND the text file is ignored

### Requirement: Storage walking honours `.nomedia`

The gallery SHALL skip any folder that contains a `.nomedia` file, and SHALL skip dot-files and
dot-folders, except when walking the trash and vault folders themselves.

#### Scenario: A `.nomedia` folder is skipped

- GIVEN a folder containing a `.nomedia` file and a photo
- WHEN its parent is scanned
- THEN the photo is not listed
- AND the rest of the parent is still listed

#### Scenario: Dot-entries are ignored

- GIVEN a folder containing `.thumbnails/` and `.nomedia.jpg`
- WHEN the folder is scanned
- THEN neither entry becomes an item

#### Scenario: Trash and vault are scanned despite leading dots

- GIVEN the trash folder `.GalleryTrash` and the vault folder `.GalleryHidden`
- WHEN the library is scanned
- THEN their contents are classified instead of being skipped as dot-folders

### Requirement: App-private storage is excluded, shared app media is included

The gallery SHALL NOT descend into `Android/data` or `Android/obb` at a scan root, and SHALL descend
into `Android/media` at a scan root so media received by other apps is listed.

#### Scenario: WhatsApp photos are listed

- GIVEN a scan root with `Android/media/com.whatsapp/WhatsApp/Media/` containing a photo
- WHEN the library is scanned
- THEN the photo is listed
- AND nothing under `Android/data` is listed

### Requirement: Newest first ordering

The gallery SHALL order discovered items by file modification time, newest first.

#### Scenario: Order follows modification time

- GIVEN three photos whose modification times are 2024-01-01, 2024-03-01 and 2023-12-31
- WHEN the library is listed
- THEN the order is 2024-03-01, 2024-01-01, 2023-12-31

### Requirement: Date-grouped grid with readable headings

The gallery SHALL group consecutive items of the same local calendar day under one heading and
SHALL label that heading `Today`, `Yesterday`, `<day> <month>` within the current year, or
`<day> <month> <year>` otherwise.

#### Scenario: Today and yesterday

- GIVEN an item modified today and one modified yesterday
- WHEN the grid is built
- THEN the first heading is `Today`
- AND the second heading is `Yesterday`

#### Scenario: Older day shows the year only when it differs

- GIVEN an item from 5 March of the current year and one from 5 March 2019
- WHEN the grid is built
- THEN the headings are `5 March` and `5 March 2019`

### Requirement: Grid density is configurable and bounded

The gallery SHALL show a configurable number of columns per row, defaulting to 5 when nothing is
stored, accepting only values from 3 to 10, and persisting the choice as a `columns=<n>` line in the
app's settings file.

#### Scenario: Stored value is used

- GIVEN a settings file containing `columns=7`
- WHEN the grid is rendered
- THEN each row holds up to 7 cells

#### Scenario: Out-of-range value is clamped

- GIVEN a settings file containing `columns=42`
- WHEN the setting is read
- THEN the grid uses 10 columns

### Requirement: Trashed and hidden items stay out of the library

The gallery SHALL exclude trashed items, vault items and items opened from other apps from the
Photos view and from normal albums; trashed items appear only in the Trash album, vault items only
in the Hidden album after unlock, and opened items only in the external album.

#### Scenario: Trashed and hidden items disappear from Photos

- GIVEN one visible photo, one trashed photo and one hidden photo
- WHEN the Photos view is built
- THEN only the visible photo is listed

#### Scenario: Video item shows its duration

- GIVEN a video whose length is 95 seconds
- WHEN it is shown in the grid
- THEN its duration label reads `1:35`
