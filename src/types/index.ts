export interface KeyValue {
  key: string;
  value: string;
  enabled: boolean;
}

export interface FormDataField {
  key: string;
  value: string;
  type: 'text' | 'file';
  enabled: boolean;
  file_path?: string;
}

export interface AuthData {
  username?: string;
  password?: string;
  token?: string;
  key?: string;
  value_field?: string;
}

export interface HttpRequest {
  method: string;
  url: string;
  headers: KeyValue[];
  body: string;
  body_type: string;
  auth_type: string;
  auth_data: AuthData;
  form_data?: FormDataField[];
}

export interface Cookie {
  name: string;
  value: string;
  domain?: string;
  path?: string;
}

export interface HttpResponse {
  status: number;
  status_text: string;
  headers: Record<string, string>;
  body: string;
  response_time: number;
  size: number;
  cookies: Cookie[];
}

export interface Collection {
  id: string;
  name: string;
  parent_id?: string;
  is_folder: boolean;
  created_at: string;
}

export interface Request {
  id: string;
  collection_id: string;
  name: string;
  method: string;
  url: string;
  headers: string; // JSON string
  body: string;
  body_type: string;
  auth_type: string;
  auth_data: string; // JSON string
  created_at: string;
  updated_at: string;
}

export interface Environment {
  id: string;
  name: string;
  variables: string; // JSON string
  is_active: boolean;
  created_at: string;
}

export interface EnvironmentVariable {
  key: string;
  value: string;
}

export interface History {
  id: string;
  method: string;
  url: string;
  status: number;
  response_time: number;
  created_at: string;
}

export interface TreeNode {
  key: string;
  label: string;
  data: Collection | Request;
  icon?: string;
  children?: TreeNode[];
  type: 'folder' | 'request';
}
