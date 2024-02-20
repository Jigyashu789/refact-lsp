use async_trait::async_trait;
use itertools::Itertools;
use strsim::normalized_damerau_levenshtein;

use crate::at_commands::at_commands::{AtCommandsContext, AtParam};

#[derive(Debug)]
pub struct AtParamFilePath {
    pub name: String,
}

impl AtParamFilePath {
    pub fn new() -> Self {
        Self {
            name: "file_path".to_string()
        }
    }
}

#[async_trait]
impl AtParam for AtParamFilePath {
    fn name(&self) -> &String {
        &self.name
    }
    async fn is_value_valid(&self, value: &String, context: &AtCommandsContext) -> bool {
        match *context.global_context.read().await.vec_db.lock().await {
            Some(ref db) => {
                let index_file_paths = db.get_indexed_file_paths().await;
                let index_file_paths = index_file_paths.lock().await;
                index_file_paths.iter().any(|path| path.to_str().unwrap() == value)
            }
            None => false,
        }
    }
    async fn complete(&self, value: &String, context: &AtCommandsContext, top_n: usize) -> Vec<String> {
        match *context.global_context.read().await.vec_db.lock().await {
            Some(ref db) => {
                let index_file_paths = db.get_indexed_file_paths().await;
                let index_file_paths = index_file_paths.lock().await;
                let mapped_paths = index_file_paths.iter().map(|f| {
                    (
                        f,
                        normalized_damerau_levenshtein(
                            if value.starts_with("/") {
                                f.to_str().unwrap()
                            } else {
                                f.file_name().unwrap().to_str().unwrap()
                            },
                            &value.to_string(),
                        )
                    )
                });
                let sorted_paths = mapped_paths
                    .sorted_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
                    .rev()
                    .map(|(path, _)| path.to_str().unwrap().to_string())
                    .take(top_n)
                    .collect::<Vec<String>>();
                return sorted_paths;
            }
            None => vec![]
        }
    }
    fn complete_if_valid(&self) -> bool {
        false
    }
}


#[derive(Debug)]
pub struct AtParamSymbolPathQuery {
    pub name: String,
}

impl AtParamSymbolPathQuery {
    pub fn new() -> Self {
        Self {
            name: "context_file".to_string()
        }
    }
}

#[async_trait]
impl AtParam for AtParamSymbolPathQuery {
    fn name(&self) -> &String {
        &self.name
    }
    async fn is_value_valid(&self, _: &String, _: &AtCommandsContext) -> bool {
        return true;
    }
    async fn complete(&self, value: &String, context: &AtCommandsContext, top_n: usize) -> Vec<String> {
        let ast_module_ptr = context.global_context.read().await.ast_module.clone();
        let index_paths = match *ast_module_ptr.lock().await {
            Some(ref ast) => ast.get_indexed_symbol_paths().await,
            None => vec![]
        };
        let mapped_paths = index_paths.iter().map(|f| {
            let filename = f.split("::").dropping(1).into_iter().join("::");
            (
                f,
                normalized_damerau_levenshtein(
                    if value.starts_with("/") {
                        f
                    } else {
                        &filename
                    },
                    &value.to_string(),
                )
            )
        });
        let sorted_paths = mapped_paths
            .sorted_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
            .rev()
            .map(|(path, _)| path.clone())
            .take(top_n)
            .collect::<Vec<String>>();
        return sorted_paths;
    }
    fn complete_if_valid(&self) -> bool {
        true
    }
}


#[derive(Debug)]
pub struct AtParamSymbolReferencePathQuery {
    pub name: String,
}

impl AtParamSymbolReferencePathQuery {
    pub fn new() -> Self {
        Self {
            name: "context_file".to_string()
        }
    }
}

#[async_trait]
impl AtParam for AtParamSymbolReferencePathQuery {
    fn name(&self) -> &String {
        &self.name
    }
    async fn is_value_valid(&self, _: &String, _: &AtCommandsContext) -> bool {
        return true;
    }
    async fn complete(&self, value: &String, context: &AtCommandsContext, top_n: usize) -> Vec<String> {
        let ast_module_ptr = context.global_context.read().await.ast_module.clone();
        let index_paths = match *ast_module_ptr.lock().await {
            Some(ref ast) => ast.get_indexed_references().await,
            None => vec![]
        };
        let mapped_paths = index_paths.iter().map(|f| {
            let filename = f.split("::").dropping(1).into_iter().join("::");
            (
                f,
                normalized_damerau_levenshtein(
                    if value.starts_with("/") {
                        f
                    } else {
                        &filename
                    },
                    &value.to_string(),
                )
            )
        });
        let sorted_paths = mapped_paths
            .sorted_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
            .rev()
            .map(|(path, _)| path.clone())
            .take(top_n)
            .collect::<Vec<String>>();
        return sorted_paths;
    }
    fn complete_if_valid(&self) -> bool {
        true
    }
}

