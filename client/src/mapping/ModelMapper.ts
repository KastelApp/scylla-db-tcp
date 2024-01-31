interface docFields<T> {
    fields?: (keyof T)[];
    allowFiltering?: boolean;
}

interface FindDocInfo<T> {
    fields?: (keyof T)[];
    orderBy?: { [key: string]: string; };
    limit?: number;
    allowFiltering?: boolean;
};

interface InsertDocInfo<T> {
    fields?: (keyof T)[];
    ifNotExists?: boolean;
}

type UpdateDocInfo<T> = {
    fields?: (keyof T)[];
    ifExists?: boolean;
    orderBy?: { [key: string]: string; };
    limit?: number;
    deleteOnlyColumns?: boolean;
};

type RemoveDocInfo<T> = {
    fields?: (keyof T)[];
    ifExists?: boolean;
    deleteOnlyColumns?: boolean;
};

interface ModelMapper<T = any> {
    get(doc: Partial<T>, docInfo: docFields<T>): T | null;
    find(doc: Partial<T>, docInfo: docFields<T>): T | null;
    findAll(doc: FindDocInfo<T>): T[];
    insert(doc: Partial<T>, docInfo: InsertDocInfo<T>): T | null;
    update(doc: Partial<T>, docInfo: UpdateDocInfo<T>): T | null;
    remove(doc: Partial<T>, docInfo: RemoveDocInfo<T>): boolean;
}

export type {
    ModelMapper,
    docFields,
    FindDocInfo,
    InsertDocInfo,
    UpdateDocInfo,
    RemoveDocInfo
}