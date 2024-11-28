export interface Settings {
  openai_api_key: string;
  pinecone_api_key: string;
  pinecone_index_host: string;
  pinecone_namespace: string;
}

export const DEFAULT_SETTINGS: Settings = {
  openai_api_key: '',
  pinecone_api_key: '',
  pinecone_index_host: '',
  pinecone_namespace: 'book'
};
