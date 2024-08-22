use std::path::Path;

use anyhow::Result;
use log::debug;
use rusqlite::{Connection, OpenFlags};

use crate::sync::data::DateAdded;

use super::data::{Album, AlbumArtist, AlbumName, AlbumTsv, Playlist, ReleaseYear, TrackCount};

pub fn get_the_most_recent_starred_albums(db: &Path, offset: usize) -> Result<Vec<AlbumTsv>> {
    debug!(
        "🪵 get_the_most_recent_starred_albums called with db={:#?}, offset={}",
        db, offset
    );
    let limit = 25;

    let conn = Connection::open_with_flags(
        db,
        OpenFlags::SQLITE_OPEN_READ_ONLY
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    let mut stmt = conn.prepare(SELECT_STARRED_ALBUMS)?;

    let albums = stmt.query_map(
        &[
            (":limit", &limit.to_string()),
            (":offset", &offset.to_string()),
        ],
        |row| {
            Ok(Album::new(
                AlbumName(row.get(1)?),
                AlbumArtist(row.get(0)?),
                TrackCount(row.get::<usize, u16>(2)?),
                ReleaseYear(row.get::<usize, String>(3)?.parse::<i32>().unwrap()),
                DateAdded(row.get(4)?),
                Playlist(row.get(5)?),
            ))
        },
    )?;

    let mut x: Vec<AlbumTsv> = Vec::new();

    for album in albums {
        x.push(album?.to_tsv_entry());
    }

    Ok(x)
}

const SELECT_STARRED_ALBUMS: &str = "select GROUP_CONCAT(artist, '; ') as artists, album, track_count, release_date, added_at, playlist
from
(
    select
        art.name as artist,
        a.name as album,
        a.id as album_id,
        a.total_tracks as track_count,
        substr(a.release_date, 1, 4) as release_date,
        pt.added_at,
        p.name as playlist,
        p.id as playlist_id
    from Album a
    join albumartist aa on aa.album_id = a.id
    join artist art on art.id = aa.artist_id
    join track t on t.album_id = a.id
    join playlisttrack pt on pt.track_id = t.id
    join playlist p on p.id = pt.playlist_id
    where p.name like 'starred%'
    group by a.id, art.id, p.id
    order by p.id, a.id, art.name
)
group by album_id, playlist_id
order by added_at DESC
limit :limit OFFSET :offset";
