export interface BlockType {
  role: string
  value: {
    id: string
    version: number
    type: "page" | "text" | "header" | "column_list" | "column"
    properties: {
      [key: string]: any[][]
    }
    content?: string[]
    permisssions?: {
      role: string
      type: string
      allow_search_engine_indexing: boolean
    }
    format?: {
      column_ratio?: number
    }
    created_time: number
    last_edited_time: number
    parent_id: string
    parent_table: string
    alive: boolean
    created_by_table: string
    created_by_id: string
    last_edited_by_table: string
    last_edited_by_id: string
  }
}

export interface NotionUserType {
  role: string
  value: {
    id: string
    version: number
    email: string
    given_name: string
    family_name: string
    profile_photo: string
    onboarding_complete: boolean
    mobile_onboarding_complete: boolean
    clipper_onboarding_complete: boolean
  }
}

export interface RecordMapType {
  block: {
    [key: string]: BlockType
  }
  notion_user: {
    [key: string]: NotionUserType
  }
}

export interface LoadPageChunkData {
  recordMap: RecordMapType
  cursor: {
    stack: any[]
  }
}