import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type {
  Collection,
  Request,
  Environment,
  History,
  HttpRequest,
  HttpResponse,
  TreeNode,
  EnvironmentVariable,
} from '../types';

export const useAppStore = defineStore('app', () => {
  // State
  const collections = ref<Collection[]>([]);
  const requests = ref<Map<string, Request[]>>(new Map());
  const environments = ref<Environment[]>([]);
  const activeEnvironment = ref<Environment | null>(null);
  const history = ref<History[]>([]);

  // Tab management
  interface RequestTab {
    id: string;
    name: string;
    request: HttpRequest;
    response: HttpResponse | null;
    requestId: string | null;
    isLoading: boolean;
    isDirty: boolean;
  }

  const openTabs = ref<RequestTab[]>([]);
  const activeTabIndex = ref(0);

  // Initialize with one empty tab
  if (openTabs.value.length === 0) {
    openTabs.value.push({
      id: crypto.randomUUID(),
      name: 'New Request',
      request: {
        method: 'GET',
        url: '',
        headers: [],
        body: '',
        body_type: 'none',
        auth_type: 'none',
        auth_data: {},
        form_data: [],
      },
      response: null,
      requestId: null,
      isLoading: false,
      isDirty: false,
    });
  }

  // Computed properties for current tab with getters/setters
  const currentRequest = computed({
    get: () => {
      return openTabs.value[activeTabIndex.value]?.request || {
        method: 'GET',
        url: '',
        headers: [],
        body: '',
        body_type: 'none',
        auth_type: 'none',
        auth_data: {},
        form_data: [],
      };
    },
    set: (value: HttpRequest) => {
      if (openTabs.value[activeTabIndex.value]) {
        openTabs.value[activeTabIndex.value].request = value;
        openTabs.value[activeTabIndex.value].isDirty = true;
      }
    }
  });

  const currentResponse = computed({
    get: () => {
      return openTabs.value[activeTabIndex.value]?.response || null;
    },
    set: (value: HttpResponse | null) => {
      if (openTabs.value[activeTabIndex.value]) {
        openTabs.value[activeTabIndex.value].response = value;
      }
    }
  });

  const selectedRequestId = computed({
    get: () => {
      return openTabs.value[activeTabIndex.value]?.requestId || null;
    },
    set: (value: string | null) => {
      if (openTabs.value[activeTabIndex.value]) {
        openTabs.value[activeTabIndex.value].requestId = value;
      }
    }
  });

  const isLoading = computed({
    get: () => {
      return openTabs.value[activeTabIndex.value]?.isLoading || false;
    },
    set: (value: boolean) => {
      if (openTabs.value[activeTabIndex.value]) {
        openTabs.value[activeTabIndex.value].isLoading = value;
      }
    }
  });

  // Computed
  const collectionTree = computed<TreeNode[]>(() => {
    const buildTree = (parentId: string | null = null): TreeNode[] => {
      const items: TreeNode[] = [];

      // Add folders
      const folders = collections.value.filter(
        (c) => c.is_folder && (c.parent_id === parentId || (!c.parent_id && !parentId))
      );

      folders.forEach((folder) => {
        const children = buildTree(folder.id);
        const folderRequests = requests.value.get(folder.id) || [];

        folderRequests.forEach((req) => {
          children.push({
            key: req.id,
            label: req.name,
            data: req,
            icon: 'pi pi-file',
            type: 'request',
          });
        });

        items.push({
          key: folder.id,
          label: folder.name,
          data: folder,
          icon: 'pi pi-folder',
          children,
          type: 'folder',
        });
      });

      // Add root-level requests
      const rootRequests = collections.value.filter(
        (c) => !c.is_folder && (c.parent_id === parentId || (!c.parent_id && !parentId))
      );

      rootRequests.forEach((collection) => {
        const collectionRequests = requests.value.get(collection.id) || [];
        collectionRequests.forEach((req) => {
          items.push({
            key: req.id,
            label: req.name,
            data: req,
            icon: 'pi pi-file',
            type: 'request',
          });
        });
      });

      return items;
    };

    return buildTree();
  });

  // Collections
  async function loadCollections() {
    try {
      const result = await invoke<Collection[]>('get_all_collections');
      collections.value = result;

      // Load requests for each collection
      for (const collection of result) {
        await loadRequestsByCollection(collection.id);
      }
    } catch (error) {
      console.error('Failed to load collections:', error);
    }
  }

  async function createCollection(name: string, parentId?: string, isFolder = false) {
    try {
      const collection: Collection = {
        id: crypto.randomUUID(),
        name,
        parent_id: parentId,
        is_folder: isFolder,
        created_at: new Date().toISOString(),
      };
      await invoke('create_collection', { collection });
      await loadCollections();
    } catch (error) {
      console.error('Failed to create collection:', error);
    }
  }

  async function deleteCollection(id: string) {
    try {
      await invoke('delete_collection', { id });
      await loadCollections();
    } catch (error) {
      console.error('Failed to delete collection:', error);
    }
  }

  // Requests
  async function loadRequestsByCollection(collectionId: string) {
    try {
      const result = await invoke<Request[]>('get_requests_by_collection', {
        collectionId,
      });
      requests.value.set(collectionId, result);
    } catch (error) {
      console.error('Failed to load requests:', error);
    }
  }

  async function saveCurrentRequest(name: string, collectionId: string) {
    try {
      const now = new Date().toISOString();
      const request: Request = {
        id: selectedRequestId.value || crypto.randomUUID(),
        collection_id: collectionId,
        name,
        method: currentRequest.value.method,
        url: currentRequest.value.url,
        headers: JSON.stringify(currentRequest.value.headers),
        body: currentRequest.value.body,
        body_type: currentRequest.value.body_type,
        auth_type: currentRequest.value.auth_type,
        auth_data: JSON.stringify(currentRequest.value.auth_data),
        created_at: now,
        updated_at: now,
      };
      await invoke('save_request', { request });
      await loadRequestsByCollection(collectionId);
      selectedRequestId.value = request.id;

      // Update current tab
      if (openTabs.value[activeTabIndex.value]) {
        openTabs.value[activeTabIndex.value].name = name;
        openTabs.value[activeTabIndex.value].isDirty = false;
      }
    } catch (error) {
      console.error('Failed to save request:', error);
    }
  }

  async function loadRequest(id: string, openInNewTab: boolean = false) {
    try {
      const request = await invoke<Request>('get_request', { id });
      if (request) {
        // Check if request is already open in a tab
        const existingTabIndex = openTabs.value.findIndex(tab => tab.requestId === id);

        if (existingTabIndex !== -1) {
          // Switch to existing tab
          activeTabIndex.value = existingTabIndex;
        } else if (openInNewTab || openTabs.value[activeTabIndex.value]?.requestId || openTabs.value[activeTabIndex.value]?.isDirty) {
          // Open in new tab if requested, or if current tab has a saved request, or if current tab is dirty
          createNewTab(request.name, request);
        } else {
          // Load into current tab
          selectedRequestId.value = id;
          currentRequest.value = {
            method: request.method,
            url: request.url,
            headers: JSON.parse(request.headers || '[]'),
            body: request.body,
            body_type: request.body_type,
            auth_type: request.auth_type,
            auth_data: JSON.parse(request.auth_data || '{}'),
            form_data: [],
          };
          if (openTabs.value[activeTabIndex.value]) {
            openTabs.value[activeTabIndex.value].name = request.name;
            openTabs.value[activeTabIndex.value].isDirty = false;
          }
        }
      }
    } catch (error) {
      console.error('Failed to load request:', error);
    }
  }

  async function deleteRequest(id: string, collectionId: string) {
    try {
      await invoke('delete_request', { id });
      await loadRequestsByCollection(collectionId);
      if (selectedRequestId.value === id) {
        selectedRequestId.value = null;
        resetCurrentRequest();
      }
    } catch (error) {
      console.error('Failed to delete request:', error);
    }
  }

  function resetCurrentRequest() {
    currentRequest.value = {
      method: 'GET',
      url: '',
      headers: [],
      body: '',
      body_type: 'none',
      auth_type: 'none',
      auth_data: {},
      form_data: [],
    };
    currentResponse.value = null;
    selectedRequestId.value = null;
  }

  // Tab Management Functions
  function createNewTab(name: string = 'New Request', request?: Request) {
    const newTab: RequestTab = {
      id: crypto.randomUUID(),
      name: request?.name || name,
      request: request ? {
        method: request.method,
        url: request.url,
        headers: JSON.parse(request.headers || '[]'),
        body: request.body,
        body_type: request.body_type,
        auth_type: request.auth_type,
        auth_data: JSON.parse(request.auth_data || '{}'),
        form_data: [],
      } : {
        method: 'GET',
        url: '',
        headers: [],
        body: '',
        body_type: 'none',
        auth_type: 'none',
        auth_data: {},
        form_data: [],
      },
      response: null,
      requestId: request?.id || null,
      isLoading: false,
      isDirty: false,
    };

    openTabs.value.push(newTab);
    activeTabIndex.value = openTabs.value.length - 1;
  }

  function closeTab(index: number) {
    if (openTabs.value.length === 1) {
      // Don't close the last tab, just reset it
      resetCurrentRequest();
      openTabs.value[0].name = 'New Request';
      openTabs.value[0].isDirty = false;
      return;
    }

    openTabs.value.splice(index, 1);

    // Adjust active tab index if needed
    if (activeTabIndex.value >= openTabs.value.length) {
      activeTabIndex.value = openTabs.value.length - 1;
    } else if (activeTabIndex.value > index) {
      activeTabIndex.value--;
    }
  }

  function switchTab(index: number) {
    if (index >= 0 && index < openTabs.value.length) {
      activeTabIndex.value = index;
    }
  }

  function updateTabName(index: number, name: string) {
    if (openTabs.value[index]) {
      openTabs.value[index].name = name;
    }
  }

  // HTTP Request
  async function sendRequest() {
    try {
      isLoading.value = true;
      currentResponse.value = null;

      // Interpolate variables in URL
      let url = currentRequest.value.url;
      if (activeEnvironment.value) {
        url = await invoke<string>('interpolate_variables', { text: url });
      }

      // Interpolate variables in headers
      const headers = [...currentRequest.value.headers];
      for (let i = 0; i < headers.length; i++) {
        if (activeEnvironment.value) {
          headers[i].value = await invoke<string>('interpolate_variables', {
            text: headers[i].value,
          });
        }
      }

      // Interpolate variables in body
      let body = currentRequest.value.body;
      if (activeEnvironment.value && body) {
        body = await invoke<string>('interpolate_variables', { text: body });
      }

      const requestToSend: HttpRequest = {
        ...currentRequest.value,
        url,
        headers,
        body,
      };

      const response = await invoke<HttpResponse>('send_http_request', {
        request: requestToSend,
      });

      currentResponse.value = response;

      // Add to history
      await addHistory({
        id: crypto.randomUUID(),
        method: currentRequest.value.method,
        url: currentRequest.value.url,
        status: response.status,
        response_time: response.response_time,
        created_at: new Date().toISOString(),
      });
    } catch (error) {
      console.error('Request failed:', error);
      throw error;
    } finally {
      isLoading.value = false;
    }
  }

  // Environments
  async function loadEnvironments() {
    try {
      const result = await invoke<Environment[]>('get_all_environments');
      environments.value = result;
      const active = result.find((e) => e.is_active);
      activeEnvironment.value = active || null;
    } catch (error) {
      console.error('Failed to load environments:', error);
    }
  }

  async function saveEnvironment(
    name: string,
    variables: EnvironmentVariable[],
    isActive = false
  ) {
    try {
      const env: Environment = {
        id: crypto.randomUUID(),
        name,
        variables: JSON.stringify(variables),
        is_active: isActive,
        created_at: new Date().toISOString(),
      };
      await invoke('save_environment', { env });
      await loadEnvironments();
    } catch (error) {
      console.error('Failed to save environment:', error);
    }
  }

  async function updateEnvironment(
    id: string,
    name: string,
    variables: EnvironmentVariable[]
  ) {
    try {
      const env = environments.value.find((e) => e.id === id);
      if (env) {
        const updated: Environment = {
          ...env,
          name,
          variables: JSON.stringify(variables),
        };
        await invoke('save_environment', { env: updated });
        await loadEnvironments();
      }
    } catch (error) {
      console.error('Failed to update environment:', error);
    }
  }

  async function setActiveEnvironment(id: string) {
    try {
      const env = environments.value.find((e) => e.id === id);
      if (env) {
        const updated: Environment = { ...env, is_active: true };
        await invoke('save_environment', { env: updated });
        await loadEnvironments();
      }
    } catch (error) {
      console.error('Failed to set active environment:', error);
    }
  }

  async function deleteEnvironment(id: string) {
    try {
      await invoke('delete_environment', { id });
      await loadEnvironments();
    } catch (error) {
      console.error('Failed to delete environment:', error);
    }
  }

  // History
  async function loadHistory(limit = 50) {
    try {
      const result = await invoke<History[]>('get_history', { limit });
      history.value = result;
    } catch (error) {
      console.error('Failed to load history:', error);
    }
  }

  async function addHistory(historyItem: History) {
    try {
      await invoke('add_history', { history: historyItem });
      await loadHistory();
    } catch (error) {
      console.error('Failed to add history:', error);
    }
  }

  async function clearHistory() {
    try {
      await invoke('clear_history');
      history.value = [];
    } catch (error) {
      console.error('Failed to clear history:', error);
    }
  }

  // Code Generation
  function generateCurl(): string {
    const parts: string[] = ['curl'];

    // Method
    if (currentRequest.value.method !== 'GET') {
      parts.push(`-X ${currentRequest.value.method}`);
    }

    // Headers
    const enabledHeaders = currentRequest.value.headers.filter(h => h.enabled && h.key);
    for (const header of enabledHeaders) {
      parts.push(`-H '${header.key}: ${header.value}'`);
    }

    // Auth
    if (currentRequest.value.auth_type === 'basic' && currentRequest.value.auth_data.username) {
      parts.push(`-u '${currentRequest.value.auth_data.username}:${currentRequest.value.auth_data.password || ''}'`);
    } else if (currentRequest.value.auth_type === 'bearer' && currentRequest.value.auth_data.token) {
      parts.push(`-H 'Authorization: Bearer ${currentRequest.value.auth_data.token}'`);
    } else if (currentRequest.value.auth_type === 'apikey' && currentRequest.value.auth_data.key) {
      parts.push(`-H '${currentRequest.value.auth_data.key}: ${currentRequest.value.auth_data.value_field}'`);
    }

    // Body
    if (currentRequest.value.body && currentRequest.value.body_type !== 'none') {
      const body = currentRequest.value.body.replace(/'/g, "'\\''");
      parts.push(`-d '${body}'`);

      // Add content-type header if not already present
      const hasContentType = enabledHeaders.some(h =>
        h.key.toLowerCase() === 'content-type'
      );
      if (!hasContentType && currentRequest.value.body_type === 'json') {
        parts.push(`-H 'Content-Type: application/json'`);
      }
    }

    // URL (always last)
    parts.push(`'${currentRequest.value.url}'`);

    return parts.join(' \\\n  ');
  }

  async function copyAsCurl(): Promise<void> {
    const curl = generateCurl();
    await navigator.clipboard.writeText(curl);
  }

  // Import/Export
  async function importPostmanCollection(jsonContent: string): Promise<void> {
    try {
      const imported = await invoke<any>('import_postman_collection', { jsonContent });
      await invoke('save_imported_collection', { imported });
      await loadCollections();
    } catch (error) {
      console.error('Failed to import collection:', error);
      throw error;
    }
  }

  async function exportCollection(collectionId: string): Promise<string> {
    try {
      const json = await invoke<string>('export_collection', { collectionId });
      return json;
    } catch (error) {
      console.error('Failed to export collection:', error);
      throw error;
    }
  }

  // Initialize
  async function initialize() {
    await Promise.all([loadCollections(), loadEnvironments(), loadHistory()]);
  }

  return {
    // State
    collections,
    requests,
    environments,
    activeEnvironment,
    history,
    currentRequest,
    currentResponse,
    selectedRequestId,
    isLoading,

    // Tab State
    openTabs,
    activeTabIndex,

    // Computed
    collectionTree,

    // Actions
    loadCollections,
    createCollection,
    deleteCollection,
    loadRequestsByCollection,
    saveCurrentRequest,
    loadRequest,
    deleteRequest,
    resetCurrentRequest,
    sendRequest,
    loadEnvironments,
    saveEnvironment,
    updateEnvironment,
    setActiveEnvironment,
    deleteEnvironment,
    loadHistory,
    addHistory,
    clearHistory,
    generateCurl,
    copyAsCurl,
    importPostmanCollection,
    exportCollection,
    initialize,

    // Tab Actions
    createNewTab,
    closeTab,
    switchTab,
    updateTabName,
  };
});
