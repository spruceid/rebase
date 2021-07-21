import { Hasher } from '../provider/hasher';
import { ContentViewer } from '../provider/contentStorage';

// StoredClaim represents the underlying data stored in the blockchain.
export type StorageEntry<Hash, Ref> = [Hash, Ref];

// Converts a NewClaim to it's storage primative.
// Throws if the hash does not match the content at the given reference.
export async function storedFromRef<Content, Hash, Ref>(
  ref: Ref,
  hasher: Hasher<Content, Hash>,
  reader: ContentViewer<Content, Ref>,
): Promise<StorageEntry<Hash, Ref>> {
  const content = await reader(ref);
  const hash = await hasher(content);
  return [hash, ref];
}

// Entry is the normalized data structure for the consuming application representing both
// the data stored in Hash Storage (hash, ref) and the data contained at the ref
// (content) along with a tag indicating if the hash of the content matches the storage.
// Generic to allow mixing and matching.
// Why not a class? No async constructors.
export type Entry<Content, Hash, Ref> = {
  // The content found when calling ContentViewer(ref)
  content: Content;
  // The hash stored on the in HashStorage, meant to match Hasher(content)
  hash: Hash;
  // The reference where content can be found if passed to ContentViewer()
  ref: Ref;
  // Does hasher(content) === hash?
  valid: boolean;
};

// Converts a StorageEntry to an Entry.  Does not throw on bad hash,
// but notes it in the valid field.
export async function entryFromStorageEntry<Content, Hash, Ref>(
  storedClaim: StorageEntry<Hash, Ref>,
  hasher: Hasher<Content, Hash>,
  reader: ContentViewer<Content, Ref>,
): Promise<Entry<Content, Hash, Ref>> {
  const [hash, ref] = storedClaim;
  const content = await reader(ref);
  const valid = await hasher(content) === hash;
  return {
    content,
    hash,
    ref,
    valid,
  };
}

// Storage represents the data structure contained in the smart contract.
export type Storage<Hash, Ref> = Array<StorageEntry<Hash, Ref>>;

// ContentList represents the Content pointed to from the smart contract in the
// form of the Wrapper, which attests to validity of the Hash
export type ContentList<Content, Hash, Ref> = Array<Entry<Content, Hash, Ref>>;

// NextContent represents data to be changed in content and hash chain storage
export type NextContent<Content, Ref> = Array<[Content, Ref]>;
