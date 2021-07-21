import { Storage } from '../abstraction/storage';

// Retrieves the Hash Storage from a given Location, false for does not exist.
export type HashViewer<Hash, Location, Ref> = (location: Location) =>
Promise<Array<Storage<Hash, Ref>>>;

export interface HashControl<Hash, Location, Ref> {
  // Identifier of the hash control authorized entity.
  id: string;
  // Add entries to hash storage
  create(newStorage: Storage<Hash, Ref>, location: Location): Promise<void>;

  // Return the location of the authorized hash storage.
  getLocation(): Location;

  // If the process of getting the location allows immedaite access to storage, this can be passed
  // Otherwise will be composed out of provider.locate(provider.getLocation())
  getStorage?(): Promise<Array<Storage<Hash, Ref>>>;

  // Find any hash storage given the location.
  locate: HashViewer<Hash, Location, Ref>;

  // Create new hash storage from initial claims
  originate(initialStorage: Storage<Hash, Ref>): Promise<void>;

  // Remove the given claims from hash storage
  remove(targetStorage: Storage<Hash, Ref>, location: Location): Promise<void>;

  // Update the given claims in hash storage.
  // If not provided, will be implemented as remove then create
  update?(
    prevStorage: Storage<Hash, Ref>,
    nextStorage: Storage<Hash, Ref>,
    location: Location
  ): Promise<void>;
}
