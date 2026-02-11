<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useAppStore } from '../stores/app';
import Dialog from 'primevue/dialog';
import Button from 'primevue/button';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Badge from 'primevue/badge';

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:visible', value: boolean): void;
}>();

const store = useAppStore();

const isVisible = computed({
  get: () => props.visible,
  set: (value) => emit('update:visible', value),
});

function getStatusSeverity(status: number) {
  if (status >= 200 && status < 300) return 'success';
  if (status >= 300 && status < 400) return 'warning';
  return 'danger';
}

function formatDate(dateString: string) {
  const date = new Date(dateString);
  return date.toLocaleString();
}

async function clearHistory() {
  await store.clearHistory();
}

onMounted(() => {
  store.loadHistory();
});
</script>

<template>
  <Dialog
    v-model:visible="isVisible"
    header="Request History"
    :modal="true"
    :style="{ width: '800px' }"
  >
    <div class="history-panel">
      <div class="history-header">
        <p>Recent requests</p>
        <Button
          label="Clear History"
          icon="pi pi-trash"
          severity="danger"
          text
          @click="clearHistory"
        />
      </div>

      <DataTable
        :value="store.history"
        :paginator="true"
        :rows="10"
        class="history-table"
      >
        <Column field="method" header="Method" style="width: 100px">
          <template #body="{ data }">
            <Badge :value="data.method" severity="info" />
          </template>
        </Column>
        <Column field="url" header="URL" />
        <Column field="status" header="Status" style="width: 100px">
          <template #body="{ data }">
            <Badge
              :value="data.status"
              :severity="getStatusSeverity(data.status)"
            />
          </template>
        </Column>
        <Column field="response_time" header="Time" style="width: 100px">
          <template #body="{ data }">
            {{ data.response_time }}ms
          </template>
        </Column>
        <Column field="created_at" header="Date" style="width: 180px">
          <template #body="{ data }">
            {{ formatDate(data.created_at) }}
          </template>
        </Column>
      </DataTable>
    </div>
  </Dialog>
</template>

<style scoped>
.history-panel {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.history-header p {
  margin: 0;
  color: var(--text-color-secondary);
}

.history-table {
  font-size: 0.875rem;
}
</style>
