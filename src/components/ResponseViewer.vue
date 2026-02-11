<script setup lang="ts">
import { computed } from 'vue';
import { useAppStore } from '../stores/app';
import TabView from 'primevue/tabview';
import TabPanel from 'primevue/tabpanel';
import Badge from 'primevue/badge';
import Button from 'primevue/button';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import hljs from 'highlight.js/lib/core';
import json from 'highlight.js/lib/languages/json';
import xml from 'highlight.js/lib/languages/xml';
import 'highlight.js/styles/github-dark.css';

hljs.registerLanguage('json', json);
hljs.registerLanguage('xml', xml);

const store = useAppStore();

const statusSeverity = computed(() => {
  if (!store.currentResponse) return 'secondary';
  const status = store.currentResponse.status;
  if (status >= 200 && status < 300) return 'success';
  if (status >= 300 && status < 400) return 'warning';
  return 'danger';
});

const formattedBody = computed(() => {
  if (!store.currentResponse) return '';

  try {
    const parsed = JSON.parse(store.currentResponse.body);
    const formatted = JSON.stringify(parsed, null, 2);
    return hljs.highlight(formatted, { language: 'json' }).value;
  } catch {
    // Try XML
    if (store.currentResponse.body.trim().startsWith('<')) {
      return hljs.highlight(store.currentResponse.body, { language: 'xml' }).value;
    }
    return store.currentResponse.body;
  }
});

const headersArray = computed(() => {
  if (!store.currentResponse) return [];
  return Object.entries(store.currentResponse.headers).map(([key, value]) => ({
    key,
    value,
  }));
});

const formattedSize = computed(() => {
  if (!store.currentResponse) return '0 B';
  const size = store.currentResponse.size;
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)} KB`;
  return `${(size / (1024 * 1024)).toFixed(2)} MB`;
});

async function copyResponse() {
  if (store.currentResponse) {
    await navigator.clipboard.writeText(store.currentResponse.body);
  }
}
</script>

<template>
  <div class="response-viewer">
    <div v-if="!store.currentResponse" class="empty-state">
      <i class="pi pi-inbox" style="font-size: 3rem"></i>
      <p>Send a request to see the response</p>
    </div>

    <div v-else class="response-content">
      <div class="response-header">
        <div class="status-info">
          <Badge
            :value="`${store.currentResponse.status} ${store.currentResponse.status_text}`"
            :severity="statusSeverity"
            size="large"
          />
          <span class="metric">
            <i class="pi pi-clock"></i>
            {{ store.currentResponse.response_time }}ms
          </span>
          <span class="metric">
            <i class="pi pi-file"></i>
            {{ formattedSize }}
          </span>
        </div>
        <Button
          label="Copy"
          icon="pi pi-copy"
          text
          @click="copyResponse"
        />
      </div>

      <TabView class="response-tabs">
        <TabPanel header="Body" :value="0">
          <div class="response-body">
            <pre v-html="formattedBody"></pre>
          </div>
        </TabPanel>

        <TabPanel header="Headers" :value="1">
          <DataTable :value="headersArray" class="headers-table">
            <Column field="key" header="Key" />
            <Column field="value" header="Value" />
          </DataTable>
        </TabPanel>

        <TabPanel header="Cookies" :value="2" v-if="store.currentResponse.cookies.length > 0">
          <DataTable :value="store.currentResponse.cookies" class="cookies-table">
            <Column field="name" header="Name" />
            <Column field="value" header="Value" />
            <Column field="domain" header="Domain" />
            <Column field="path" header="Path" />
          </DataTable>
        </TabPanel>
      </TabView>
    </div>
  </div>
</template>

<style scoped>
.response-viewer {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 1rem;
  overflow: hidden;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-color-secondary);
  gap: 1rem;
}

.response-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 1rem;
  overflow: hidden;
}

.response-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: var(--surface-50);
  border-radius: 6px;
  flex-shrink: 0;
}

.status-info {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.metric {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-color-secondary);
}

.response-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.response-tabs :deep(.p-tabview-panels) {
  flex: 1;
  overflow: auto;
}

.response-body {
  overflow: auto;
  height: 100%;
}

.response-body pre {
  margin: 0;
  padding: 1rem;
  background: var(--surface-ground);
  border-radius: 6px;
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.headers-table,
.cookies-table {
  font-size: 0.875rem;
}
</style>
