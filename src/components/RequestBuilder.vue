<script setup lang="ts">
import { ref, computed } from 'vue';
import { useAppStore } from '../stores/app';
import TabView from 'primevue/tabview';
import TabPanel from 'primevue/tabpanel';
import Dropdown from 'primevue/dropdown';
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Textarea from 'primevue/textarea';
import RadioButton from 'primevue/radiobutton';
import type { KeyValue } from '../types';

const store = useAppStore();

const activeTab = ref(0);

const methods = ['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD', 'OPTIONS'];
const bodyTypes = [
  { label: 'None', value: 'none' },
  { label: 'JSON', value: 'json' },
  { label: 'Form Data', value: 'form' },
  { label: 'Raw', value: 'raw' },
];
const authTypes = [
  { label: 'None', value: 'none' },
  { label: 'Basic Auth', value: 'basic' },
  { label: 'Bearer Token', value: 'bearer' },
  { label: 'API Key', value: 'apikey' },
];

const newHeader = ref<KeyValue>({ key: '', value: '', enabled: true });
const newQueryParam = ref<KeyValue>({ key: '', value: '', enabled: true });

// Helper to check if value contains variables
function hasVariables(value: string): boolean {
  return value.includes('{{') && value.includes('}}');
}

// Get environment variable value
function getVariableValue(value: string): string {
  if (!store.activeEnvironment || !hasVariables(value)) {
    return '';
  }

  try {
    const variables = JSON.parse(store.activeEnvironment.variables || '[]');
    // Extract variable name from {{variableName}}
    const match = value.match(/\{\{([^}]+)\}\}/);
    if (match) {
      const varName = match[1].trim();
      const variable = variables.find((v: any) => v.key === varName);
      if (variable) {
        return variable.value;
      }
    }
  } catch {
    // Ignore parsing errors
  }

  return '';
}

const queryParams = computed({
  get: () => {
    try {
      const url = new URL(store.currentRequest.url, 'http://localhost');
      const params: KeyValue[] = [];
      url.searchParams.forEach((value, key) => {
        params.push({ key, value, enabled: true });
      });
      return params;
    } catch {
      return [];
    }
  },
  set: (params: KeyValue[]) => {
    try {
      const url = new URL(store.currentRequest.url, 'http://localhost');
      url.search = '';
      params
        .filter((p) => p.enabled && p.key)
        .forEach((p) => {
          url.searchParams.append(p.key, p.value);
        });
      store.currentRequest.url = url.toString().replace('http://localhost', '');
    } catch {
      // Invalid URL
    }
  },
});

function addHeader() {
  if (newHeader.value.key) {
    store.currentRequest.headers.push({ ...newHeader.value });
    newHeader.value = { key: '', value: '', enabled: true };
  }
}

function removeHeader(index: number) {
  store.currentRequest.headers.splice(index, 1);
}

function addQueryParam() {
  if (newQueryParam.value.key) {
    const params = [...queryParams.value, { ...newQueryParam.value }];
    queryParams.value = params;
    newQueryParam.value = { key: '', value: '', enabled: true };
  }
}

function removeQueryParam(index: number) {
  const params = [...queryParams.value];
  params.splice(index, 1);
  queryParams.value = params;
}

async function handleSend() {
  try {
    await store.sendRequest();
  } catch (error) {
    console.error('Failed to send request:', error);
  }
}

async function handleCopyAsCurl() {
  try {
    await store.copyAsCurl();
  } catch (error) {
    console.error('Failed to copy as cURL:', error);
  }
}

function addFormField() {
  if (!store.currentRequest.form_data) {
    store.currentRequest.form_data = [];
  }
  store.currentRequest.form_data.push({
    key: '',
    value: '',
    type: 'text',
    enabled: true,
  });
}

function removeFormField(index: number) {
  if (store.currentRequest.form_data) {
    store.currentRequest.form_data.splice(index, 1);
  }
}

async function selectFile(index: number) {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      multiple: false,
      directory: false,
    });

    if (selected && store.currentRequest.form_data) {
      const field = store.currentRequest.form_data[index];
      field.value = selected;
      // Store the file path for the backend
      (field as any).file_path = selected;
    }
  } catch (error) {
    console.error('Failed to select file:', error);
  }
}
</script>

<template>
  <div class="request-builder">
    <div class="request-line">
      <Dropdown v-model="store.currentRequest.method" :options="methods" class="method-dropdown" />
      <InputText v-model="store.currentRequest.url" placeholder="Enter URL (use {{variable}} for environment variables)"
        class="url-input" />
      <Button label="Send" icon="pi pi-send" @click="handleSend" :loading="store.isLoading" severity="success" />
      <Button label="Copy as cURL" icon="pi pi-copy" @click="handleCopyAsCurl" text severity="secondary" />
    </div>

    <TabView v-model:activeIndex="activeTab" class="request-tabs">
        <TabPanel header="Query Params" :value="0">
          <DataTable :value="queryParams" class="params-table">
            <Column field="enabled" header="">
              <template #body="{ data }">
                <input type="checkbox" v-model="data.enabled" @change="queryParams = [...queryParams]" />
              </template>
            </Column>
            <Column field="key" header="Key">
              <template #body="{ data }">
                <InputText v-model="data.key" @blur="queryParams = [...queryParams]" class="w-full" />
              </template>
            </Column>
            <Column field="value" header="Value">
              <template #body="{ data }">
                <div class="value-cell">
                  <InputText v-model="data.value" @blur="queryParams = [...queryParams]" class="w-full" />
                  <span v-if="hasVariables(data.value) && getVariableValue(data.value)" class="resolved-value">
                    → {{ getVariableValue(data.value) }}
                  </span>
                </div>
              </template>
            </Column>
            <Column header="">
              <template #body="{ index }">
                <Button icon="pi pi-trash" text severity="danger" @click="removeQueryParam(index)" />
              </template>
            </Column>
          </DataTable>
          <div class="add-row">
            <input type="checkbox" v-model="newQueryParam.enabled" />
            <InputText v-model="newQueryParam.key" placeholder="Key" />
            <InputText v-model="newQueryParam.value" placeholder="Value" />
            <Button icon="pi pi-plus" text @click="addQueryParam" />
          </div>
        </TabPanel>

        <TabPanel header="Headers" :value="1">
          <DataTable :value="store.currentRequest.headers" class="params-table">
            <Column field="enabled" header="">
              <template #body="{ data }">
                <input type="checkbox" v-model="data.enabled" />
              </template>
            </Column>
            <Column field="key" header="Key">
              <template #body="{ data }">
                <InputText v-model="data.key" class="w-full" />
              </template>
            </Column>
            <Column field="value" header="Value">
              <template #body="{ data }">
                <div class="value-cell">
                  <InputText v-model="data.value" class="w-full" />
                  <span v-if="hasVariables(data.value) && getVariableValue(data.value)" class="resolved-value">
                    → {{ getVariableValue(data.value) }}
                  </span>
                </div>
              </template>
            </Column>
            <Column header="">
              <template #body="{ index }">
                <Button icon="pi pi-trash" text severity="danger" @click="removeHeader(index)" />
              </template>
            </Column>
          </DataTable>
          <div class="add-row">
            <input type="checkbox" v-model="newHeader.enabled" />
            <InputText v-model="newHeader.key" placeholder="Key" />
            <InputText v-model="newHeader.value" placeholder="Value" />
            <Button icon="pi pi-plus" text @click="addHeader" />
          </div>
        </TabPanel>

        <TabPanel header="Body" :value="2">
          <div class="body-type-selector">
            <div v-for="type in bodyTypes" :key="type.value" class="radio-option">
              <RadioButton v-model="store.currentRequest.body_type" :inputId="type.value" :value="type.value" />
              <label :for="type.value">{{ type.label }}</label>
            </div>
          </div>

          <!-- Form Data -->
          <div v-if="store.currentRequest.body_type === 'form'">
            <DataTable :value="store.currentRequest.form_data" class="params-table">
              <Column field="enabled" header="">
                <template #body="{ data }">
                  <input type="checkbox" v-model="data.enabled" />
                </template>
              </Column>
              <Column field="key" header="Key">
                <template #body="{ data }">
                  <InputText v-model="data.key" class="w-full" />
                </template>
              </Column>
              <Column field="value" header="Value">
                <template #body="{ data }">
                  <InputText v-if="data.type === 'text'" v-model="data.value" class="w-full" />
                  <span v-else>{{ data.value || 'No file selected' }}</span>
                </template>
              </Column>
              <Column field="type" header="Type">
                <template #body="{ data }">
                  <Dropdown v-model="data.type" :options="['text', 'file']" class="w-full" />
                </template>
              </Column>
              <Column header="">
                <template #body="{ index }">
                  <Button v-if="store.currentRequest.form_data && store.currentRequest.form_data[index]?.type === 'file'" icon="pi pi-folder-open" text severity="secondary" @click="selectFile(index)" />
                  <Button icon="pi pi-trash" text severity="danger" @click="removeFormField(index)" />
                </template>
              </Column>
            </DataTable>
            <div class="add-row">
              <input type="checkbox" checked disabled />
              <Button label="Add Field" icon="pi pi-plus" text @click="addFormField" />
            </div>
          </div>

          <!-- JSON/Raw Body -->
          <Textarea v-else-if="store.currentRequest.body_type !== 'none'" v-model="store.currentRequest.body" :placeholder="store.currentRequest.body_type === 'json'
              ? 'Enter JSON body'
              : 'Enter request body'
            " rows="10" class="body-editor" />
        </TabPanel>

        <TabPanel header="Auth" :value="3">
          <div class="auth-selector">
            <div v-for="type in authTypes" :key="type.value" class="radio-option">
              <RadioButton v-model="store.currentRequest.auth_type" :inputId="'auth-' + type.value"
                :value="type.value" />
              <label :for="'auth-' + type.value">{{ type.label }}</label>
            </div>
          </div>

          <div v-if="store.currentRequest.auth_type === 'basic'" class="auth-fields">
            <div class="field">
              <label>Username</label>
              <InputText v-model="store.currentRequest.auth_data.username" class="w-full" />
            </div>
            <div class="field">
              <label>Password</label>
              <InputText v-model="store.currentRequest.auth_data.password" type="password" class="w-full" />
            </div>
          </div>

          <div v-if="store.currentRequest.auth_type === 'bearer'" class="auth-fields">
            <div class="field">
              <label>Token</label>
              <InputText v-model="store.currentRequest.auth_data.token" class="w-full" />
            </div>
          </div>

          <div v-if="store.currentRequest.auth_type === 'apikey'" class="auth-fields">
            <div class="field">
              <label>Key</label>
              <InputText v-model="store.currentRequest.auth_data.key" class="w-full" />
            </div>
            <div class="field">
              <label>Value</label>
              <InputText v-model="store.currentRequest.auth_data.value_field" class="w-full" />
            </div>
          </div>
        </TabPanel>
    </TabView>
  </div>
</template>

<style scoped>
.request-builder {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  height: 100%;
  padding: 1rem;
  overflow: hidden;
}

.request-line {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-shrink: 0;
}

.method-dropdown {
  width: 120px;
}

.url-input {
  flex: 1;
}

.request-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.request-tabs :deep(.p-tabview-panels) {
  flex: 1;
  overflow: auto;
}

.params-table {
  margin-bottom: 1rem;
}

.value-cell {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.resolved-value {
  font-size: 0.75rem;
  color: var(--primary-400);
  font-family: 'Courier New', monospace;
  padding-left: 0.5rem;
  opacity: 0.9;
}

.add-row {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.add-row input[type='checkbox'] {
  width: 20px;
}

.add-row .p-inputtext {
  flex: 1;
}

.body-type-selector,
.auth-selector {
  display: flex;
  gap: 1.5rem;
  margin-bottom: 1rem;
}

.radio-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.body-editor {
  width: 100%;
  font-family: 'Courier New', monospace;
}

.auth-fields {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-width: 500px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.field label {
  font-weight: 500;
  font-size: 0.875rem;
}
</style>
