import {
  ContentList, NextContent, Storage,
} from './abstraction/storage';
import { Hasher } from './provider/hasher';
import { ContentControl } from './provider/contentStorage';
import { HashControl } from './provider/hashStorage';
import ControlClient from './ControlClient';
import ViewClient from './ViewClient';

export type ControlOpts<Content, Hash, Location, Ref> = {
  clientId: string;
  hasher: Hasher<Content, Hash>;
  contentControl: ContentControl<Content, Ref>;
  hashControl: HashControl<Hash, Location, Ref>;
};

export type AppOpts<Content, Hash, Location, Ref> = {
  appId: string;
  controlOptsList: Array<ControlOpts<Content, Hash, Location, Ref>>;
};

type AppMap<Content, Hash, Location, Ref> = {
  [index: string]: ControlClientMap<Content, Hash, Location, Ref>;
};

type ControlClientMap<Content, Hash, Location, Ref> = {
  [index: string]: ControlClient<Content, Hash, Location, Ref>;
};

function toControlClientMap<Content, Hash, Location, Ref>(
  controlOptsList: Array<ControlOpts<Content, Hash, Location, Ref>>,
): ControlClientMap<Content, Hash, Location, Ref> {
  const clientMap: ControlClientMap<Content, Hash, Location, Ref> = {};
  controlOptsList.forEach((opts) => {
    clientMap[opts.clientId] = new ControlClient(
      opts.hasher,
      opts.contentControl,
      opts.hashControl,
    );
  });

  return clientMap;
}

export class MultiClient<Content, Hash, Location, Ref> {
  publicClient: ViewClient<Content, Hash, Location, Ref> | false;

  appMap: AppMap<Content, Hash, Location, Ref> = {};

  constructor(
    appOpts: Array<AppOpts<Content, Hash, Location, Ref>>,
    publicClient?: ViewClient<Content, Hash, Location, Ref>,
  ) {
    this.publicClient = publicClient || false;
    appOpts.forEach((opts) => {
      this.appMap[opts.appId] = toControlClientMap(opts.controlOptsList);
    });
  }

  // Some boilerplate reducing internal access methods:
  private getClientMap(appId: string): ControlClientMap<Content, Hash, Location, Ref> {
    const app = this.appMap[appId];
    if (!app) {
      throw new Error(`Cannot find client for app: ${appId}`);
    }
    return app;
  }

  getClient(appId: string, clientId: string): ControlClient<Content, Hash, Location, Ref> {
    const app = this.getClientMap(appId);
    const entry = app[clientId];
    if (!entry) {
      throw new Error(`Cannot find client for id: ${clientId}`);
    }

    return entry;
  }

  // Add more client to an existing multi-client, new client at the same ID will override.
  addClients(optsList: Array<AppOpts<Content, Hash, Location, Ref>>): void {
    optsList.forEach((opts) => {
      const existingEntry = this.appMap[opts.appId];

      if (existingEntry) {
        const nextClientMap: ControlClientMap<Content, Hash, Location, Ref> = {};
        opts.controlOptsList.forEach((controlOpts) => {
          nextClientMap[controlOpts.clientId] = new ControlClient(
            controlOpts.hasher,
            controlOpts.contentControl,
            controlOpts.hashControl,
          );
        });

        this.appMap[opts.appId] = Object.assign(
          existingEntry,
          nextClientMap,
        );
        return;
      }

      this.appMap[opts.appId] = toControlClientMap(opts.controlOptsList);
    });
  }

  // Get content list from given location using the public client.
  publicRead(location: Location): Promise<Array<ContentList<Content, Hash, Ref>>> {
    if (!this.publicClient) {
      throw new Error('No public read available');
    }

    return this.publicClient.read(location);
  }

  // Get content list from a given app and client id.
  readOwn(
    appId: string,
    clientId: string,
  ): Promise<Array<ContentList<Content, Hash, Ref>>> {
    return this.getClient(appId, clientId).readOwn();
  }

  // Originate new hash storage with references to content storage.
  async originateRef(
    appId: string,
    clientId: string,
    newContentList: Array<Ref>,
  ): Promise<void> {
    return this.getClient(appId, clientId).originateRef(newContentList);
  }

  // Originate new hash storage with content not yet in content storage.
  async originateContent(
    appId: string,
    clientId: string,
    newContentList: Array<Content>,
  ): Promise<void> {
    return this.getClient(appId, clientId).originateContent(newContentList);
  }

  // NOTE: void could be a passable result type, ir we wanted to capture tx number.
  // Add claims to a contract given a location using given app and client.
  async create(appId: string, clientId: string, refList: Array<Content>): Promise<void> {
    return this.getClient(appId, clientId).create(refList);
  }

  // NOTE: void could be a passable result type, ir we wanted to capture tx number.
  // Remove claims to a contract given a location using given app and client.
  async remove(appId: string, clientId: string, prevStorage: Storage<Hash, Ref>): Promise<void> {
    return this.getClient(appId, clientId).remove(prevStorage);
  }

  // NOTE: This one is subtler, since it may be composed out of 2 tx.
  // Update claims to a contract using given app and client.
  async update(
    appId: string,
    clientId: string,
    nextContent: NextContent<Content, Ref>,
  ): Promise<void> {
    await this.getClient(appId, clientId).update(nextContent);
  }
}
