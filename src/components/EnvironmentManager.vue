<script setup lang="ts">
import { ref, computed } from 'vue';
import { useAppStore } from '../stores/app';
import Dialog from 'primevue/dialog';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import type { EnvironmentVariable } from '../types';

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:visible', value: boolean): void;
}>();

const store = useAppStore();

const envName = ref('');
const variables = ref<EnvironmentVariable[]>([]);
const newVariable = ref<EnvironmentVariable>({ key: '', value: '' });
const editingEnvId = ref<string | null>(null);

const isVisible = computed({
  get: () => props.visible,
  set: (value) => emit('update:visible', value),
});

function addVariable() {
  if (newVariable.value.key) {
    variables.value.push({ ...newVariable.value });
    newVariable.value = { key: '', value: '' };
  }
}

function removeVariable(index: number) {
  variables.value.splice(index, 1);
}

async function saveEnvironment() {
  if (!envName.value) return;

  if (editingEnvId.value) {
    // Update existing environment
    await store.updateEnvironment(editingEnvId.value, envName.value, variables.value);
  } else {
    // Create new environment
    await store.saveEnvironment(envName.value, variables.value, false);
  }

  resetForm();
  isVisible.value = false;
}

function resetForm() {
  editingEnvId.value = null;
  envName.value = '';
  variables.value = [];
  newVariable.value = { key: '', value: '' };
}

function loadEnvironment(envId: string) {
  const env = store.environments.find((e) => e.id === envId);
  if (env) {
    editingEnvId.value = envId;
    envName.value = env.name;
    variables.value = JSON.parse(env.variables || '[]');
  }
}

async function deleteEnvironment(id: string) {
  await store.deleteEnvironment(id);
}
</script>

<template>
  <Dialog
    v-model:visible="isVisible"
    header="Environment Manager"
    :modal="true"
    :style="{ width: '900px', maxHeight: '80vh' }"
  >
    <div class="env-manager">
      <div class="env-list">
        <h4>Environments</h4>
        <div class="env-items">
          <div
            v-for="env in store.environments"
            :key="env.id"
            class="env-item"
            :class="{ active: env.is_active }"
          >
            <div class="env-info" @click="loadEnvironment(env.id)">
              <div class="env-header">
                <span class="env-name">{{ env.name }}</span>
                <span v-if="env.is_active" class="active-badge">Active</span>
              </div>
              <span class="env-var-count">
                {{ JSON.parse(env.variables || '[]').length }} variable{{
                  JSON.parse(env.variables || '[]').length !== 1 ? 's' : ''
                }}
              </span>
            </div>
            <div class="env-actions">
              <Button
                icon="pi pi-pencil"
                text
                size="small"
                @click="loadEnvironment(env.id)"
                v-tooltip.left="'Edit'"
              />
              <Button
                v-if="!env.is_active"
                label="Activate"
                text
                size="small"
                @click="store.setActiveEnvironment(env.id)"
              />
              <Button
                icon="pi pi-trash"
                text
                severity="danger"
                size="small"
                @click="deleteEnvironment(env.id)"
                v-tooltip.left="'Delete'"
              />
            </div>
          </div>
        </div>
      </div>

      <div class="env-editor">
        <div class="editor-header">
          <h4>{{ editingEnvId ? `Editing: ${envName}` : 'New Environment' }}</h4>
          <Button
            v-if="editingEnvId"
            label="New Environment"
            icon="pi pi-plus"
            text
            size="small"
            @click="resetForm"
          />
        </div>

        <div class="field">
          <label>Environment Name</label>
          <InputText
            v-model="envName"
            placeholder="e.g., Development, Production"
            class="w-full"
          />
        </div>

        <div class="field">
          <label>Variables</label>
          <DataTable :value="variables" class="variables-table">
            <Column field="key" header="Key">
              <template #body="{ data }">
                <InputText v-model="data.key" class="w-full" />
              </template>
            </Column>
            <Column field="value" header="Value">
              <template #body="{ data }">
                <InputText v-model="data.value" class="w-full" />
              </template>
            </Column>
            <Column header="">
              <template #body="{ index }">
                <Button
                  icon="pi pi-trash"
                  text
                  severity="danger"
                  @click="removeVariable(index)"
                />
              </template>
            </Column>
          </DataTable>

          <div class="add-variable-section">
            <label class="add-variable-label">Add New Variable</label>
            <div class="add-variable">
              <InputText
                v-model="newVariable.key"
                placeholder="Key"
                @keyup.enter="addVariable"
              />
              <InputText
                v-model="newVariable.value"
                placeholder="Value"
                @keyup.enter="addVariable"
              />
              <Button
                icon="pi pi-plus"
                @click="addVariable"
                v-tooltip.top="'Add Variable'"
              />
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <Button label="Cancel" text @click="isVisible = false" />
      <Button label="Save Environment" @click="saveEnvironment" />
    </template>
  </Dialog>
</template>

<style scoped>
.env-manager {
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: 2rem;
  min-height: 500px;
  max-height: 60vh;
}

.env-list h4 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  font-weight: 600;
}

.env-editor h4 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
}

.env-items {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  overflow-y: auto;
  max-height: calc(60vh - 100px);
}

.env-item {
  padding: 0.75rem;
  border: 1px solid var(--surface-border);
  border-radius: 6px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.env-item.active {
  background: var(--primary-50);
  border-color: var(--primary-200);
}

.env-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  flex: 1;
  cursor: pointer;
  min-width: 0;
}

.env-info:hover {
  opacity: 0.8;
}

.env-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.env-name {
  font-weight: 500;
}

.env-var-count {
  font-size: 0.75rem;
  color: var(--text-color-secondary);
}

.active-badge {
  font-size: 0.75rem;
  color: var(--primary-color);
  font-weight: 600;
  background: var(--primary-100);
  padding: 0.125rem 0.375rem;
  border-radius: 4px;
}

.env-actions {
  display: flex;
  gap: 0.25rem;
}

.env-editor {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.field label {
  font-weight: 500;
  font-size: 0.875rem;
}

.variables-table {
  margin-bottom: 0.5rem;
}

.add-variable-section {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.add-variable-label {
  font-weight: 500;
  font-size: 0.875rem;
  color: var(--text-color-secondary);
}

.add-variable {
  display: flex;
  gap: 0.5rem;
}

.add-variable .p-inputtext {
  flex: 1;
}
</style>
