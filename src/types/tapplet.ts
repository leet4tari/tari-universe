import { TariPermission } from "@tari-project/tarijs/dist/providers/tari_universe"

//TODO tauri commands cammelCase to snake
export type RegisteredTapplet = {
  id: string
  registry_id: string
  package_name: string
  display_name: string
  author_name: string
  author_website: string
  about_summary: string
  about_description: string
  category: string
}

export type RegisteredTappletWithAssets = RegisteredTapplet & {
  logoAddr: string
  backgroundAddr: string
}

export type InstalledTapplet = {
  id: string
  tapplet_id: string
  tapplet_version_id: string
}

export interface InstalledTappletWithName {
  installed_tapplet: InstalledTapplet
  display_name: string
  installed_version: string
  latest_version: string
}

export interface DevTapplet {
  id: string //TODO change to number
  package_name: string
  endpoint: string
  display_name: string
  about_summary: string
  about_description: string
}

export interface TappletVersion {
  id: string
  tapplet_id: string
  version: string
  integrity: string
  registry_url: string
  logo_url: string
}

export interface RegisteredTappletWithVersion {
  id: string
  registered_tapp: RegisteredTapplet
  tapp_version: TappletVersion
}

export interface TappletAudit {
  id: string
  tapplet_id: string
  auditor: string
  report_url: string
}

export interface LaunchedTappResult {
  endpoint: string
  permissions: TappletPermissions
}

export type SupportedChain = "MAINNET" | "STAGENET" | "NEXTNET" | ""
export interface TappletConfig {
  packageName: string
  version: string
  supportedChain: SupportedChain[]
  permissions: TappletPermissions
}

export interface TappletPermissions {
  requiredPermissions: TariPermission[]
  optionalPermissions: TariPermission[]
}
