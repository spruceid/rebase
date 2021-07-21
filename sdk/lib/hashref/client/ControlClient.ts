import {
  ContentList, Entry, entryFromStorageEntry,
  NextContent, Storage, StorageEntry, storedFromRef,
} from './abstraction/storage';
import { Hasher } from './provider/hasher';
import { ContentControl } from './provider/contentStorage';
import { HashControl } from './provider/hashStorage';

export default class ControlClient<Content, Hash, Location, Ref> {
  hasher: Hasher<Content, Hash>;

  contentControl: ContentControl<Content, Ref>;

  hashControl: HashControl<Hash, Location, Ref>;

  constructor(
    hasher: Hasher<Content, Hash>,
    contentControl: ContentControl<Content, Ref>,
    hashControl: HashControl<Hash, Location, Ref>,
  ) {
    this.hasher = hasher;
    this.contentControl = contentControl;
    this.hashControl = hashControl;
  }

  // Compose now to later map
  private fromStoredToEntry(s: StorageEntry<Hash, Ref>): Promise<Entry<Content, Hash, Ref>> {
    return entryFromStorageEntry(s, this.hasher, this.contentControl.read);
  }

  // Compose to later map
  private fromRefToStored(ref: Ref): Promise<StorageEntry<Hash, Ref>> {
    return storedFromRef(ref, this.hasher, this.contentControl.read);
  }

  // Get content list from given location.
  async read(location: Location): Promise<Array<ContentList<Content, Hash, Ref>>> {
    const storageList = await this.hashControl.locate(location);

    return Promise.all(
      storageList.map(
        (storage) => Promise.all(
          storage.map(this.fromStoredToEntry.bind(this)),
        ),
      ),
    ) as Promise<Array<ContentList<Content, Hash, Ref>>>;
  }

  async readOwn(): Promise<Array<ContentList<Content, Hash, Ref>>> {
    let storageList: Array<Storage<Hash, Ref>> = [];

    if (this.hashControl.getStorage) {
      storageList = await this.hashControl.getStorage();
    } else {
      const location = this.hashControl.getLocation();
      if (location) {
        storageList = await this.hashControl.locate(location);
      }
    }

    return Promise.all(
      storageList.map(
        (storage) => Promise.all(storage.map(this.fromStoredToEntry.bind(this))),
      ),
    ) as Promise<Array<ContentList<Content, Hash, Ref>>>;
  }

  // Originate new hash storage with references to content storage.
  async originateRef(refList: Array<Ref>): Promise<void> {
    return this.hashControl.originate(
      await Promise.all(refList.map(this.fromRefToStored.bind(this))) as Storage<Hash, Ref>,
    );
  }

  // Originate new hash storage with content not yet in content storage.
  async originateContent(contentList: Array<Content>): Promise<void> {
    return this.originateRef(await this.contentControl.create(contentList));
  }

  // NOTE: void could be a passable result type?
  // Add claims to a contract given an instance using current client.
  async create(
    contentList: Array<Content>,
    location?: Location,
  ): Promise<void> {
    const refList = await this.contentControl.create(contentList);

    const prepared = await Promise.all(
      refList.map(this.fromRefToStored.bind(this)),
    ) as Storage<Hash, Ref>;

    return this.hashControl.create(prepared, (location || this.hashControl.getLocation()));
  }

  // NOTE: void could be a passable result type?
  // Remove claims to a contract given an instance using current client.
  async remove(targetStorage: Storage<Hash, Ref>, location?: Location): Promise<void> {
    await this.hashControl.remove(targetStorage, (location || this.hashControl.getLocation()));
    await this.contentControl.remove(targetStorage.map((x) => x[1]));
  }

  // NOTE: void could be a passable result type?
  // Creates record of previous hashes for use in removal, then updates
  // content storage with new content, then either updates using hashControl.update
  // or uses hashControl.remove && hashControl.create
  async update(updatePairList: NextContent<Content, Ref>, location?: Location): Promise<void> {
    const nextHashList: Array<Hash> = [];
    const oldRefList: Array<Ref> = [];
    const prevStorage: Storage<Hash, Ref> = [];

    const l = (location || this.hashControl.getLocation());

    const nextContentList = await Promise.all(
      updatePairList.map(async (x) => {
        const [nextContent, prevRef] = x;
        const nextHash = await this.hasher(nextContent);
        const prevHash = await this.hasher(await this.contentControl.read(prevRef));

        oldRefList.push(prevRef);
        prevStorage.push([prevHash, prevRef]);
        nextHashList.push(nextHash);

        return nextContent;
      }),
    );

    const nextRefList: Array<Ref> = [];
    if (this.contentControl.update) {
      nextRefList.concat(await this.contentControl.update(
        nextContentList,
        oldRefList,
      ));
    } else {
      await this.contentControl.remove(oldRefList);
      nextRefList.concat(await this.contentControl.create(nextContentList));
    }

    if (nextRefList.length !== nextHashList.length) {
      throw new Error('Did not recieve same number of references and content');
    }

    const nextStorage: Storage<Hash, Ref> = nextRefList.map(
      (nextRef, i) => {
        const nextHash = nextHashList[i];
        if (!nextHash) {
          throw new Error('Mismatched number of references and content');
        }
        return [nextHash, nextRef];
      },
    );

    if (this.hashControl.update) {
      return this.hashControl.update(prevStorage, nextStorage, l);
    }

    await this.hashControl.remove(prevStorage, l);
    return this.hashControl.create(nextStorage, l);
  }
}
