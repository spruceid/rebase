import {
  ContentList, Entry, entryFromStorageEntry, StorageEntry,
} from './abstraction/storage';
import { Hasher } from './provider/hasher';
import { ContentViewer } from './provider/contentStorage';
import { HashViewer } from './provider/hashStorage';

export default class ViewClient<Content, Hash, Location, Ref> {
  hasher: Hasher<Content, Hash>;

  contentViewer: ContentViewer<Content, Ref>;

  hashViewer: HashViewer<Hash, Location, Ref>;

  constructor(
    hasher: Hasher<Content, Hash>,
    contentViewer: ContentViewer<Content, Ref>,
    hashViewer: HashViewer<Hash, Location, Ref>,
  ) {
    this.hasher = hasher;
    this.hashViewer = hashViewer;
    this.contentViewer = contentViewer;
  }

  fromStored(s: StorageEntry<Hash, Ref>): Promise<Entry<Content, Hash, Ref>> {
    return entryFromStorageEntry(s, this.hasher, this.contentViewer);
  }

  // Get content list from given location
  async read(location: Location): Promise<Array<ContentList<Content, Hash, Ref>>> {
    const storageList = await this.hashViewer(location);

    return Promise.all(
      storageList.map(
        async (storage) => Promise.all(storage.map(this.fromStored.bind(this))),
      ),
    ) as Promise<Array<ContentList<Content, Hash, Ref>>>;
  }
}
