<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import Dialog from 'primevue/dialog';
import Button from 'primevue/button';
import Dropdown from 'primevue/dropdown';
import Slider from 'primevue/slider';
import InputSwitch from 'primevue/inputswitch';

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();

const isVisible = computed({
  get: () => props.visible,
  set: (value) => emit('update:visible', value),
});

// Settings State
const theme = ref<'dark' | 'light'>('dark');
const fontSize = ref(14);
const fontFamily = ref<'system' | 'monospace'>('system');
const autoSave = ref(true);
const requestTimeout = ref(30);
const showLineNumbers = ref(true);

const themeOptions = [
  { label: 'Dark', value: 'dark' },
  { label: 'Light', value: 'light' },
];

const fontFamilyOptions = [
  { label: 'System Default', value: 'system' },
  { label: 'Monospace', value: 'monospace' },
];

const timeoutOptions = [
  { label: '10 seconds', value: 10 },
  { label: '30 seconds', value: 30 },
  { label: '60 seconds', value: 60 },
  { label: '120 seconds', value: 120 },
  { label: 'No timeout', value: 0 },
];

// Load settings from localStorage
function loadSettings() {
  const savedSettings = localStorage.getItem('fetchr_settings');
  if (savedSettings) {
    try {
      const settings = JSON.parse(savedSettings);
      theme.value = settings.theme || 'dark';
      fontSize.value = settings.fontSize || 14;
      fontFamily.value = settings.fontFamily || 'system';
      autoSave.value = settings.autoSave !== false;
      requestTimeout.value = settings.requestTimeout || 30;
      showLineNumbers.value = settings.showLineNumbers !== false;

      applyTheme(theme.value);
      applyFontSize(fontSize.value);
      handleFontFamilyChange();
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  }
}

// Save settings to localStorage
function saveSettings() {
  const settings = {
    theme: theme.value,
    fontSize: fontSize.value,
    fontFamily: fontFamily.value,
    autoSave: autoSave.value,
    requestTimeout: requestTimeout.value,
    showLineNumbers: showLineNumbers.value,
  };

  localStorage.setItem('fetchr_settings', JSON.stringify(settings));
  applyTheme(theme.value);
  applyFontSize(fontSize.value);
}

// Apply theme
function applyTheme(selectedTheme: 'dark' | 'light') {
  const htmlElement = document.documentElement;
  if (selectedTheme === 'dark') {
    htmlElement.classList.add('dark-mode');
    htmlElement.classList.remove('light-mode');
  } else {
    htmlElement.classList.add('light-mode');
    htmlElement.classList.remove('dark-mode');
  }
}

// Apply font size
function applyFontSize(size: number) {
  document.documentElement.style.setProperty('--base-font-size', `${size}px`);
}

// Watch for changes and auto-save
function handleThemeChange() {
  saveSettings();
}

function handleFontFamilyChange() {
  if (fontFamily.value === 'monospace') {
    document.documentElement.style.setProperty('--app-font-family', '"Courier New", monospace');
  } else {
    document.documentElement.style.setProperty('--app-font-family', '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif');
  }
  saveSettings();
}

// Reset to defaults
function resetToDefaults() {
  theme.value = 'dark';
  fontSize.value = 14;
  fontFamily.value = 'system';
  autoSave.value = true;
  requestTimeout.value = 30;
  showLineNumbers.value = true;

  saveSettings();
}

onMounted(() => {
  loadSettings();
});
</script>

<template>
  <Dialog
    v-model:visible="isVisible"
    header="Settings"
    :modal="true"
    :style="{ width: '600px' }"
  >
    <div class="settings-content">
      <!-- Appearance Section -->
      <div class="settings-section">
        <h3 class="section-title">Appearance</h3>

        <div class="setting-item">
          <label for="theme">Theme</label>
          <Dropdown
            id="theme"
            v-model="theme"
            :options="themeOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
            @change="handleThemeChange"
          />
        </div>

        <div class="setting-item">
          <div class="setting-label-row">
            <label for="fontSize">Font Size</label>
            <span class="setting-value">{{ fontSize }}px</span>
          </div>
          <Slider
            id="fontSize"
            v-model="fontSize"
            :min="10"
            :max="20"
            :step="1"
            @change="saveSettings"
          />
        </div>

        <div class="setting-item">
          <label for="fontFamily">Font Family</label>
          <Dropdown
            id="fontFamily"
            v-model="fontFamily"
            :options="fontFamilyOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
            @change="handleFontFamilyChange"
          />
        </div>

        <div class="setting-item">
          <div class="setting-toggle">
            <label for="showLineNumbers">Show Line Numbers in Code Editors</label>
            <InputSwitch
              id="showLineNumbers"
              v-model="showLineNumbers"
              @change="saveSettings"
            />
          </div>
        </div>
      </div>

      <!-- Editor Section -->
      <div class="settings-section">
        <h3 class="section-title">Editor</h3>

        <div class="setting-item">
          <div class="setting-toggle">
            <label for="autoSave">Auto-save Requests</label>
            <InputSwitch
              id="autoSave"
              v-model="autoSave"
              @change="saveSettings"
            />
          </div>
          <p class="setting-hint">Automatically save requests when switching tabs</p>
        </div>
      </div>

      <!-- Network Section -->
      <div class="settings-section">
        <h3 class="section-title">Network</h3>

        <div class="setting-item">
          <label for="requestTimeout">Request Timeout</label>
          <Dropdown
            id="requestTimeout"
            v-model="requestTimeout"
            :options="timeoutOptions"
            optionLabel="label"
            optionValue="value"
            class="w-full"
            @change="saveSettings"
          />
          <p class="setting-hint">Maximum time to wait for a response</p>
        </div>
      </div>

      <!-- Actions -->
      <div class="settings-actions">
        <Button
          label="Reset to Defaults"
          icon="pi pi-refresh"
          severity="secondary"
          text
          @click="resetToDefaults"
        />
        <Button
          label="Close"
          icon="pi pi-check"
          @click="isVisible = false"
        />
      </div>
    </div>
  </Dialog>
</template>

<style scoped>
.settings-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.settings-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.section-title {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--primary-400);
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--surface-border);
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.setting-label-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.setting-item label {
  font-weight: 500;
  color: var(--text-color);
}

.setting-value {
  font-size: 0.875rem;
  color: var(--text-color-secondary);
  font-weight: 600;
}

.setting-toggle {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: var(--surface-section);
  border-radius: 6px;
  border: 1px solid var(--surface-border);
}

.setting-hint {
  font-size: 0.875rem;
  color: var(--text-color-secondary);
  margin: 0;
  font-style: italic;
}

.settings-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 1rem;
  border-top: 1px solid var(--surface-border);
}
</style>
