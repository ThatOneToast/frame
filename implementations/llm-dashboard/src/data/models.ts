export interface Model {
  id: string;
  name: string;
  provider: string;
  quantization: string;
  contextLimit: number;
  contextUsed: number;
  tokensPerSecond: number;
  previousTps: number;
  memoryUsed: number;
  memoryTotal: number;
  status: 'healthy' | 'warning' | 'degraded';
  trend: 'up' | 'down' | 'stable';
  architecture: string;
  parameterCount: string;
}

export interface InferenceRun {
  id: string;
  modelId: string;
  modelName: string;
  provider: string;
  tokensPerSecond: number;
  latencyMs: number;
  contextSize: number;
  status: 'completed' | 'running' | 'failed';
  trend: 'up' | 'down' | 'stable';
  cost: number;
  energyWh: number;
  timestamp: string;
}

export interface DashboardMetrics {
  activeModel: Model | null;
  totalTokensGenerated: number;
  averageTps: number;
  tpsChangePercent: number;
  totalRunsToday: number;
  successRate: number;
  averageLatency: number;
  totalCostToday: number;
}

export interface TpsDataPoint {
  timestamp: string;
  tps: number;
  label: string;
}

export const models: Model[] = [
  {
    id: 'qwen3.6-coder-27b',
    name: 'Qwen3.6 Coder 27B',
    provider: 'Local (LM Studio)',
    quantization: 'Q4_K_M',
    contextLimit: 32768,
    contextUsed: 18432,
    tokensPerSecond: 42.5,
    previousTps: 38.2,
    memoryUsed: 14.2,
    memoryTotal: 24.0,
    status: 'healthy',
    trend: 'up',
    architecture: 'Transformer',
    parameterCount: '27B'
  },
  {
    id: 'qwen3.5-0.8b-draft',
    name: 'Qwen3.5 0.8B Draft',
    provider: 'Local (LM Studio)',
    quantization: 'Q8_0',
    contextLimit: 8192,
    contextUsed: 2048,
    tokensPerSecond: 185.3,
    previousTps: 178.9,
    memoryUsed: 1.1,
    memoryTotal: 2.0,
    status: 'healthy',
    trend: 'up',
    architecture: 'Transformer',
    parameterCount: '0.8B'
  },
  {
    id: 'gemma-3-27b',
    name: 'Gemma 3 27B',
    provider: 'Local (Ollama)',
    quantization: 'Q4_K_S',
    contextLimit: 16384,
    contextUsed: 12288,
    tokensPerSecond: 35.8,
    previousTps: 36.1,
    memoryUsed: 15.8,
    memoryTotal: 24.0,
    status: 'warning',
    trend: 'down',
    architecture: 'Transformer',
    parameterCount: '27B'
  },
  {
    id: 'llama-3.1-8b',
    name: 'Llama 3.1 8B',
    provider: 'Remote (OpenAI Compatible)',
    quantization: 'FP16',
    contextLimit: 131072,
    contextUsed: 45056,
    tokensPerSecond: 28.4,
    previousTps: 27.9,
    memoryUsed: 16.0,
    memoryTotal: 16.0,
    status: 'healthy',
    trend: 'stable',
    architecture: 'Transformer',
    parameterCount: '8B'
  },
  {
    id: 'deepseek-coder-6.7b',
    name: 'DeepSeek Coder 6.7B',
    provider: 'Local (LM Studio)',
    quantization: 'Q5_K_M',
    contextLimit: 16384,
    contextUsed: 8192,
    tokensPerSecond: 52.1,
    previousTps: 48.7,
    memoryUsed: 5.2,
    memoryTotal: 8.0,
    status: 'healthy',
    trend: 'up',
    architecture: 'MoE',
    parameterCount: '6.7B'
  },
  {
    id: 'phi-3.5-mini',
    name: 'Phi 3.5 Mini',
    provider: 'Local (Ollama)',
    quantization: 'Q4_K_M',
    contextLimit: 131072,
    contextUsed: 32768,
    tokensPerSecond: 95.2,
    previousTps: 92.8,
    memoryUsed: 2.4,
    memoryTotal: 4.0,
    status: 'healthy',
    trend: 'up',
    architecture: 'Transformer',
    parameterCount: '3.8B'
  }
];

export const recentRuns: InferenceRun[] = [
  {
    id: 'run-001',
    modelId: 'qwen3.6-coder-27b',
    modelName: 'Qwen3.6 Coder 27B',
    provider: 'Local',
    tokensPerSecond: 42.5,
    latencyMs: 23.5,
    contextSize: 18432,
    status: 'completed',
    trend: 'up',
    cost: 0.0,
    energyWh: 12.4,
    timestamp: '2026-06-10T14:32:00Z'
  },
  {
    id: 'run-002',
    modelId: 'gemma-3-27b',
    modelName: 'Gemma 3 27B',
    provider: 'Local',
    tokensPerSecond: 35.8,
    latencyMs: 27.9,
    contextSize: 12288,
    status: 'completed',
    trend: 'down',
    cost: 0.0,
    energyWh: 15.2,
    timestamp: '2026-06-10T14:28:00Z'
  },
  {
    id: 'run-003',
    modelId: 'llama-3.1-8b',
    modelName: 'Llama 3.1 8B',
    provider: 'Remote',
    tokensPerSecond: 28.4,
    latencyMs: 35.2,
    contextSize: 45056,
    status: 'completed',
    trend: 'stable',
    cost: 0.0042,
    energyWh: 0.0,
    timestamp: '2026-06-10T14:25:00Z'
  },
  {
    id: 'run-004',
    modelId: 'deepseek-coder-6.7b',
    modelName: 'DeepSeek Coder 6.7B',
    provider: 'Local',
    tokensPerSecond: 52.1,
    latencyMs: 19.2,
    contextSize: 8192,
    status: 'completed',
    trend: 'up',
    cost: 0.0,
    energyWh: 8.1,
    timestamp: '2026-06-10T14:20:00Z'
  },
  {
    id: 'run-005',
    modelId: 'phi-3.5-mini',
    modelName: 'Phi 3.5 Mini',
    provider: 'Local',
    tokensPerSecond: 95.2,
    latencyMs: 10.5,
    contextSize: 32768,
    status: 'running',
    trend: 'up',
    cost: 0.0,
    energyWh: 3.8,
    timestamp: '2026-06-10T14:35:00Z'
  },
  {
    id: 'run-006',
    modelId: 'qwen3.5-0.8b-draft',
    modelName: 'Qwen3.5 0.8B Draft',
    provider: 'Local',
    tokensPerSecond: 185.3,
    latencyMs: 5.4,
    contextSize: 2048,
    status: 'completed',
    trend: 'up',
    cost: 0.0,
    energyWh: 1.2,
    timestamp: '2026-06-10T14:15:00Z'
  }
];

export const tpsHistory: TpsDataPoint[] = [
  { timestamp: '14:00', tps: 38.2, label: '2:00 PM' },
  { timestamp: '14:05', tps: 39.8, label: '2:05 PM' },
  { timestamp: '14:10', tps: 41.2, label: '2:10 PM' },
  { timestamp: '14:15', tps: 40.5, label: '2:15 PM' },
  { timestamp: '14:20', tps: 42.1, label: '2:20 PM' },
  { timestamp: '14:25', tps: 43.8, label: '2:25 PM' },
  { timestamp: '14:30', tps: 42.5, label: '2:30 PM' },
  { timestamp: '14:35', tps: 44.2, label: '2:35 PM' },
  { timestamp: '14:40', tps: 43.1, label: '2:40 PM' },
  { timestamp: '14:45', tps: 45.8, label: '2:45 PM' },
  { timestamp: '14:50', tps: 44.5, label: '2:50 PM' },
  { timestamp: '14:55', tps: 46.2, label: '2:55 PM' }
];

export function getActiveModel(): Model | null {
  return models.find(m => m.id === 'qwen3.6-coder-27b') || null;
}

export function getMetrics(): DashboardMetrics {
  const activeModel = getActiveModel();
  return {
    activeModel,
    totalTokensGenerated: 1247832,
    averageTps: 42.5,
    tpsChangePercent: 11.2,
    totalRunsToday: 47,
    successRate: 97.8,
    averageLatency: 23.5,
    totalCostToday: 0.0124
  };
}

export function getTopModels(): Array<{ model: Model; improvement: number }> {
  return [
    { model: models[0], improvement: 11.2 },
    { model: models[2], improvement: -0.8 },
    { model: models[3], improvement: 1.8 },
    { model: models[5], improvement: 2.6 }
  ];
}

export function formatNumber(n: number): string {
  if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M';
  if (n >= 1000) return (n / 1000).toFixed(1) + 'K';
  return n.toFixed(1);
}

export function formatContext(used: number, limit: number): string {
  return `${formatNumber(used)} / ${formatNumber(limit)}`;
}

export function formatMemory(used: number, total: number): string {
  return `${used.toFixed(1)} GB / ${total.toFixed(1)} GB`;
}
