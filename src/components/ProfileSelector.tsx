import { Fragment } from 'react'
import { Listbox, Transition } from '@headlessui/react'
import { Check, ChevronsUpDown, Plus } from 'lucide-react'
import * as api from '@/lib/api'

interface ProfileSelectorProps {
  profiles: api.Profile[]
  activeProfile: api.Profile | null
  onProfileChange: (profileId: string) => void
  onAddProfile: () => void
}

export function ProfileSelector({
  profiles,
  activeProfile,
  onProfileChange,
  onAddProfile,
}: ProfileSelectorProps) {
  // 如果没有配置，显示特殊状态
  if (profiles.length === 0) {
    return (
      <button
        onClick={onAddProfile}
        className="w-full flex items-center justify-between px-4 py-3 bg-white border-2 border-dashed border-gray-300 rounded-lg hover:border-indigo-400 hover:bg-indigo-50 transition-all group"
      >
        <span className="text-sm text-gray-500 group-hover:text-indigo-600">
          暂无配置，点击添加
        </span>
        <Plus className="w-5 h-5 text-gray-400 group-hover:text-indigo-600" />
      </button>
    )
  }

  return (
    <Listbox value={activeProfile?.id || ''} onChange={onProfileChange}>
      <div className="relative">
        <Listbox.Button className="relative w-full cursor-pointer rounded-lg bg-white py-3 pl-4 pr-10 text-left border border-gray-300 hover:border-indigo-400 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all">
          <span className="block truncate">
            {activeProfile ? (
              <div>
                <div className="font-semibold text-gray-900">{activeProfile.name}</div>
                <div className="text-xs text-gray-500 mt-0.5">
                  {extractProvider(activeProfile.apiBaseUrl)} · {activeProfile.overrideModel || '透传模式'}
                </div>
              </div>
            ) : (
              <span className="text-gray-500">未激活配置</span>
            )}
          </span>
          <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
            <ChevronsUpDown className="h-5 w-5 text-gray-400" aria-hidden="true" />
          </span>
        </Listbox.Button>

        <Transition
          as={Fragment}
          leave="transition ease-in duration-100"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <Listbox.Options className="absolute z-10 mt-1 max-h-80 w-full overflow-auto rounded-lg bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
            {profiles.map((profile) => (
              <Listbox.Option
                key={profile.id}
                value={profile.id}
                className={({ active }) =>
                  `relative cursor-pointer select-none py-3 pl-10 pr-4 ${
                    active ? 'bg-indigo-50 text-indigo-900' : 'text-gray-900'
                  }`
                }
              >
                {({ selected }) => (
                  <>
                    <div className={`block ${selected ? 'font-semibold' : 'font-normal'}`}>
                      <div className="text-sm">{profile.name}</div>
                      <div className="text-xs text-gray-500 mt-0.5">
                        {extractProvider(profile.apiBaseUrl)} · {profile.overrideModel || '透传模式'}
                      </div>
                    </div>
                    {selected && (
                      <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-indigo-600">
                        <Check className="h-5 w-5" aria-hidden="true" />
                      </span>
                    )}
                  </>
                )}
              </Listbox.Option>
            ))}

            {/* 添加新配置选项 */}
            <div className="border-t border-gray-200 mt-1 pt-1">
              <button
                onClick={(e) => {
                  e.preventDefault()
                  onAddProfile()
                }}
                className="w-full flex items-center gap-2 px-4 py-3 text-sm text-indigo-600 hover:bg-indigo-50 transition-colors"
              >
                <Plus className="w-4 h-4" />
                <span>添加新配置</span>
              </button>
            </div>
          </Listbox.Options>
        </Transition>
      </div>
    </Listbox>
  )
}

// 从 API Base URL 提取 Provider 名称
function extractProvider(apiBaseUrl: string): string {
  try {
    const url = new URL(apiBaseUrl)
    const hostname = url.hostname.toLowerCase()

    if (hostname.includes('anthropic')) return 'Anthropic'
    if (hostname.includes('openai')) return 'OpenAI'
    if (hostname.includes('google')) return 'Google'
    if (hostname.includes('azure')) return 'Azure'
    if (hostname.includes('cohere')) return 'Cohere'
    if (hostname.includes('huggingface')) return 'HuggingFace'

    // 返回域名的主要部分
    const parts = hostname.split('.')
    if (parts.length >= 2) {
      return parts[parts.length - 2].charAt(0).toUpperCase() + parts[parts.length - 2].slice(1)
    }

    return hostname
  } catch {
    return 'Unknown'
  }
}
