// Retrieves Content given a ref, such as Kepler or HTTP request.
export type ContentViewer<Content, Ref> = (ref: Ref) => Promise<Content>;

// Full CRUD operations.
export type ContentControl<Content, Ref> = {
  // allows for lookup when using MultiClient
  id: string;
  // Given a list content to be hosted, return the refs where the content now is hosted.
  create(contentList: Array<Content>): Promise<Array<Ref>>;
  // Read a piece of content given a ref
  read: ContentViewer<Content, Ref>;
  // Remove a piece of content.
  remove: (refList: Array<Ref>) => Promise<void>;
  // Given a ref and a new piece of content, host that content at the ref.
  // If not provided, given as first remove(...) then returning the result of create(...).
  update?: (newContentList: Array<Content>, oldRefList: Array<Ref>) => Promise<Array<Ref>>;
};
