//! # 变量作用域管理
//!
//! 管理笔记本中单元格间的变量共享和作用域。

use crate::core::{Expression, Number};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use super::{CellId, NotebookError, NotebookResult};

/// 变量绑定信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableBinding {
    /// 变量名
    pub name: String,
    /// 变量值
    pub value: Expression,
    /// 定义时间
    pub defined_at: SystemTime,
    /// 定义的单元格 ID
    pub defined_in: CellId,
    /// 最后使用时间
    pub last_used: Option<SystemTime>,
    /// 使用次数
    pub usage_count: u64,
    /// 是否为常量（不可修改）
    pub is_constant: bool,
    /// 变量类型信息
    pub type_info: Option<String>,
}

impl VariableBinding {
    /// 创建新的变量绑定
    pub fn new(name: String, value: Expression, defined_in: CellId) -> Self {
        Self {
            name,
            value,
            defined_at: SystemTime::now(),
            defined_in,
            last_used: None,
            usage_count: 0,
            is_constant: false,
            type_info: None,
        }
    }
    
    /// 创建常量绑定
    pub fn new_constant(name: String, value: Expression, defined_in: CellId) -> Self {
        let mut binding = Self::new(name, value, defined_in);
        binding.is_constant = true;
        binding
    }
    
    /// 更新变量值
    pub fn update_value(&mut self, new_value: Expression, updated_in: CellId) -> NotebookResult<()> {
        if self.is_constant {
            return Err(NotebookError::Scope(format!("常量 '{}' 不能被修改", self.name)));
        }
        
        self.value = new_value;
        self.defined_in = updated_in;
        self.defined_at = SystemTime::now();
        Ok(())
    }
    
    /// 记录变量使用
    pub fn mark_used(&mut self) {
        self.last_used = Some(SystemTime::now());
        self.usage_count += 1;
    }
    
    /// 设置类型信息
    pub fn set_type_info(&mut self, type_info: String) {
        self.type_info = Some(type_info);
    }
    
    /// 获取变量摘要
    pub fn summary(&self) -> String {
        let type_str = self.type_info.as_deref().unwrap_or("未知");
        let const_str = if self.is_constant { " (常量)" } else { "" };
        format!("{}: {} = {:?}{}", self.name, type_str, self.value, const_str)
    }
}

/// 变量作用域
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableScope {
    /// 作用域名称
    pub name: String,
    /// 变量绑定映射
    variables: HashMap<String, VariableBinding>,
    /// 父作用域（用于嵌套作用域）
    parent: Option<Box<VariableScope>>,
    /// 创建时间
    pub created_at: SystemTime,
}

impl VariableScope {
    /// 创建新的作用域
    pub fn new(name: String) -> Self {
        Self {
            name,
            variables: HashMap::new(),
            parent: None,
            created_at: SystemTime::now(),
        }
    }
    
    /// 创建子作用域
    pub fn create_child(&self, name: String) -> Self {
        Self {
            name,
            variables: HashMap::new(),
            parent: Some(Box::new(self.clone())),
            created_at: SystemTime::now(),
        }
    }
    
    /// 定义变量
    pub fn define_variable(&mut self, name: String, value: Expression, defined_in: CellId) -> NotebookResult<()> {
        let binding = VariableBinding::new(name.clone(), value, defined_in);
        self.variables.insert(name, binding);
        Ok(())
    }
    
    /// 定义常量
    pub fn define_constant(&mut self, name: String, value: Expression, defined_in: CellId) -> NotebookResult<()> {
        let binding = VariableBinding::new_constant(name.clone(), value, defined_in);
        self.variables.insert(name, binding);
        Ok(())
    }
    
    /// 更新变量值
    pub fn update_variable(&mut self, name: &str, value: Expression, updated_in: CellId) -> NotebookResult<()> {
        if let Some(binding) = self.variables.get_mut(name) {
            binding.update_value(value, updated_in)?;
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.update_variable(name, value, updated_in)
        } else {
            Err(NotebookError::Scope(format!("变量 '{}' 未定义", name)))
        }
    }
    
    /// 获取变量值
    pub fn get_variable(&mut self, name: &str) -> Option<&mut VariableBinding> {
        if let Some(binding) = self.variables.get_mut(name) {
            binding.mark_used();
            Some(binding)
        } else if let Some(parent) = &mut self.parent {
            parent.get_variable(name)
        } else {
            None
        }
    }
    
    /// 检查变量是否存在
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name) || 
        self.parent.as_ref().map_or(false, |p| p.has_variable(name))
    }
    
    /// 删除变量
    pub fn remove_variable(&mut self, name: &str) -> NotebookResult<VariableBinding> {
        if let Some(binding) = self.variables.remove(name) {
            Ok(binding)
        } else {
            Err(NotebookError::Scope(format!("变量 '{}' 不存在", name)))
        }
    }
    
    /// 获取所有变量名
    pub fn get_variable_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.variables.keys().cloned().collect();
        
        if let Some(parent) = &self.parent {
            let mut parent_names = parent.get_variable_names();
            names.append(&mut parent_names);
        }
        
        names.sort();
        names.dedup();
        names
    }
    
    /// 获取本地变量（不包括父作用域）
    pub fn get_local_variables(&self) -> &HashMap<String, VariableBinding> {
        &self.variables
    }
    
    /// 清除所有变量
    pub fn clear(&mut self) {
        self.variables.clear();
    }
    
    /// 获取变量统计信息
    pub fn statistics(&self) -> ScopeStatistics {
        let mut stats = ScopeStatistics {
            total_variables: self.variables.len(),
            constants: self.variables.values().filter(|v| v.is_constant).count(),
            unused_variables: self.variables.values().filter(|v| v.usage_count == 0).count(),
            most_used_variable: None,
            least_recently_used: None,
        };
        
        // 找到使用最多的变量
        if let Some(most_used) = self.variables.values().max_by_key(|v| v.usage_count) {
            stats.most_used_variable = Some((most_used.name.clone(), most_used.usage_count));
        }
        
        // 找到最久未使用的变量
        if let Some(lru) = self.variables.values()
            .filter(|v| v.last_used.is_some())
            .min_by_key(|v| v.last_used.unwrap()) {
            stats.least_recently_used = Some(lru.name.clone());
        }
        
        stats
    }
    
    /// 导出变量到 HashMap（用于计算引擎）
    pub fn export_for_computation(&mut self) -> HashMap<String, Number> {
        let mut result = HashMap::new();
        
        // 先添加父作用域的变量
        if let Some(parent) = &mut self.parent {
            result.extend(parent.export_for_computation());
        }
        
        // 添加本地变量，覆盖父作用域的同名变量
        for (name, binding) in &mut self.variables {
            binding.mark_used();
            if let Expression::Number(num) = &binding.value {
                result.insert(name.clone(), num.clone());
            }
        }
        
        result
    }
}

/// 作用域统计信息
#[derive(Debug, Clone)]
pub struct ScopeStatistics {
    pub total_variables: usize,
    pub constants: usize,
    pub unused_variables: usize,
    pub most_used_variable: Option<(String, u64)>,
    pub least_recently_used: Option<String>,
}

/// 作用域管理器
pub struct ScopeManager {
    /// 全局作用域
    global_scope: VariableScope,
    /// 单元格作用域映射
    cell_scopes: HashMap<CellId, VariableScope>,
    /// 当前活动作用域
    current_scope: Option<CellId>,
}

impl ScopeManager {
    /// 创建新的作用域管理器
    pub fn new() -> Self {
        Self {
            global_scope: VariableScope::new("全局".to_string()),
            cell_scopes: HashMap::new(),
            current_scope: None,
        }
    }
    
    /// 创建单元格作用域
    pub fn create_cell_scope(&mut self, cell_id: CellId, name: String) {
        let scope = self.global_scope.create_child(name);
        self.cell_scopes.insert(cell_id, scope);
    }
    
    /// 设置当前作用域
    pub fn set_current_scope(&mut self, cell_id: Option<CellId>) {
        self.current_scope = cell_id;
    }
    
    /// 获取当前作用域
    pub fn get_current_scope(&mut self) -> &mut VariableScope {
        match self.current_scope {
            Some(cell_id) => {
                self.cell_scopes.get_mut(&cell_id).unwrap_or(&mut self.global_scope)
            }
            None => &mut self.global_scope,
        }
    }
    
    /// 获取全局作用域
    pub fn get_global_scope(&mut self) -> &mut VariableScope {
        &mut self.global_scope
    }
    
    /// 获取指定单元格的作用域
    pub fn get_cell_scope(&mut self, cell_id: &CellId) -> Option<&mut VariableScope> {
        self.cell_scopes.get_mut(cell_id)
    }
    
    /// 定义全局变量
    pub fn define_global_variable(&mut self, name: String, value: Expression, defined_in: CellId) -> NotebookResult<()> {
        self.global_scope.define_variable(name, value, defined_in)
    }
    
    /// 定义全局常量
    pub fn define_global_constant(&mut self, name: String, value: Expression, defined_in: CellId) -> NotebookResult<()> {
        self.global_scope.define_constant(name, value, defined_in)
    }
    
    /// 在当前作用域定义变量
    pub fn define_variable(&mut self, name: String, value: Expression, defined_in: CellId) -> NotebookResult<()> {
        self.get_current_scope().define_variable(name, value, defined_in)
    }
    
    /// 在当前作用域定义常量
    pub fn define_constant(&mut self, name: String, value: Expression, defined_in: CellId) -> NotebookResult<()> {
        self.get_current_scope().define_constant(name, value, defined_in)
    }
    
    /// 更新变量值
    pub fn update_variable(&mut self, name: &str, value: Expression, updated_in: CellId) -> NotebookResult<()> {
        self.get_current_scope().update_variable(name, value, updated_in)
    }
    
    /// 获取变量值
    pub fn get_variable(&mut self, name: &str) -> Option<&mut VariableBinding> {
        self.get_current_scope().get_variable(name)
    }
    
    /// 检查变量是否存在
    pub fn has_variable(&self, name: &str) -> bool {
        // 检查全局作用域
        if self.global_scope.has_variable(name) {
            return true;
        }
        
        // 检查当前单元格作用域
        if let Some(cell_id) = self.current_scope {
            if let Some(scope) = self.cell_scopes.get(&cell_id) {
                return scope.has_variable(name);
            }
        }
        
        false
    }
    
    /// 获取所有可见的变量名
    pub fn get_visible_variables(&self) -> Vec<String> {
        let mut names = self.global_scope.get_variable_names();
        
        if let Some(cell_id) = self.current_scope {
            if let Some(scope) = self.cell_scopes.get(&cell_id) {
                let mut cell_names = scope.get_variable_names();
                names.append(&mut cell_names);
            }
        }
        
        names.sort();
        names.dedup();
        names
    }
    
    /// 清除单元格作用域
    pub fn clear_cell_scope(&mut self, cell_id: &CellId) {
        self.cell_scopes.remove(cell_id);
    }
    
    /// 清除所有作用域
    pub fn clear_all(&mut self) {
        self.global_scope.clear();
        self.cell_scopes.clear();
        self.current_scope = None;
    }
    
    /// 导出变量用于计算
    pub fn export_for_computation(&mut self) -> HashMap<String, Number> {
        let mut result = self.global_scope.export_for_computation();
        
        if let Some(cell_id) = self.current_scope {
            if let Some(scope) = self.cell_scopes.get_mut(&cell_id) {
                result.extend(scope.export_for_computation());
            }
        }
        
        result
    }
    
    /// 获取作用域管理器统计信息
    pub fn statistics(&self) -> ManagerStatistics {
        let global_stats = self.global_scope.statistics();
        let cell_scope_count = self.cell_scopes.len();
        let total_cell_variables: usize = self.cell_scopes.values()
            .map(|scope| scope.get_local_variables().len())
            .sum();
        
        ManagerStatistics {
            global_variables: global_stats.total_variables,
            cell_scopes: cell_scope_count,
            total_cell_variables,
            total_variables: global_stats.total_variables + total_cell_variables,
        }
    }
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 管理器统计信息
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    pub global_variables: usize,
    pub cell_scopes: usize,
    pub total_cell_variables: usize,
    pub total_variables: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Number;
    use uuid::Uuid;
    
    #[test]
    fn test_variable_binding() {
        let cell_id = Uuid::new_v4();
        let mut binding = VariableBinding::new(
            "x".to_string(),
            Expression::Number(Number::from(42)),
            cell_id
        );
        
        assert_eq!(binding.name, "x");
        assert_eq!(binding.usage_count, 0);
        assert!(!binding.is_constant);
        
        binding.mark_used();
        assert_eq!(binding.usage_count, 1);
        assert!(binding.last_used.is_some());
        
        // 测试常量
        let const_binding = VariableBinding::new_constant(
            "PI".to_string(),
            Expression::Number(Number::from(3.14159)),
            cell_id
        );
        assert!(const_binding.is_constant);
    }
    
    #[test]
    fn test_variable_scope() {
        let mut scope = VariableScope::new("测试作用域".to_string());
        let cell_id = Uuid::new_v4();
        
        // 定义变量
        scope.define_variable(
            "x".to_string(),
            Expression::Number(Number::from(10)),
            cell_id
        ).unwrap();
        
        assert!(scope.has_variable("x"));
        assert!(!scope.has_variable("y"));
        
        // 获取变量
        let binding = scope.get_variable("x").unwrap();
        assert_eq!(binding.usage_count, 1);
        
        // 更新变量
        scope.update_variable(
            "x",
            Expression::Number(Number::from(20)),
            cell_id
        ).unwrap();
        
        let binding = scope.get_variable("x").unwrap();
        if let Expression::Number(Number::Integer(val)) = &binding.value {
            assert_eq!(*val, 20.into());
        } else {
            panic!("期望整数值");
        }
    }
    
    #[test]
    fn test_nested_scopes() {
        let mut parent = VariableScope::new("父作用域".to_string());
        let cell_id = Uuid::new_v4();
        
        parent.define_variable(
            "global_var".to_string(),
            Expression::Number(Number::from(100)),
            cell_id
        ).unwrap();
        
        let mut child = parent.create_child("子作用域".to_string());
        child.define_variable(
            "local_var".to_string(),
            Expression::Number(Number::from(200)),
            cell_id
        ).unwrap();
        
        // 子作用域可以访问父作用域的变量
        assert!(child.has_variable("global_var"));
        assert!(child.has_variable("local_var"));
        
        // 父作用域不能访问子作用域的变量
        assert!(parent.has_variable("global_var"));
        assert!(!parent.has_variable("local_var"));
    }
    
    #[test]
    fn test_scope_manager() {
        let mut manager = ScopeManager::new();
        let cell_id1 = Uuid::new_v4();
        let cell_id2 = Uuid::new_v4();
        
        // 定义全局变量
        manager.define_global_variable(
            "global_x".to_string(),
            Expression::Number(Number::from(1)),
            cell_id1
        ).unwrap();
        
        // 创建单元格作用域
        manager.create_cell_scope(cell_id1, "单元格1".to_string());
        manager.set_current_scope(Some(cell_id1));
        
        // 在单元格作用域定义变量
        manager.define_variable(
            "local_y".to_string(),
            Expression::Number(Number::from(2)),
            cell_id1
        ).unwrap();
        
        // 检查变量可见性
        assert!(manager.has_variable("global_x"));
        assert!(manager.has_variable("local_y"));
        
        // 切换到另一个单元格
        manager.create_cell_scope(cell_id2, "单元格2".to_string());
        manager.set_current_scope(Some(cell_id2));
        
        // 只能看到全局变量
        assert!(manager.has_variable("global_x"));
        assert!(!manager.has_variable("local_y"));
        
        // 获取统计信息
        let stats = manager.statistics();
        assert_eq!(stats.global_variables, 1);
        assert_eq!(stats.cell_scopes, 2);
    }
    
    #[test]
    fn test_constant_protection() {
        let mut scope = VariableScope::new("测试".to_string());
        let cell_id = Uuid::new_v4();
        
        // 定义常量
        scope.define_constant(
            "PI".to_string(),
            Expression::Number(Number::from(3.14159)),
            cell_id
        ).unwrap();
        
        // 尝试修改常量应该失败
        let result = scope.update_variable(
            "PI",
            Expression::Number(Number::from(3.0)),
            cell_id
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("常量"));
    }
    
    #[test]
    fn test_export_for_computation() {
        let mut manager = ScopeManager::new();
        let cell_id = Uuid::new_v4();
        
        manager.define_global_variable(
            "x".to_string(),
            Expression::Number(Number::from(10)),
            cell_id
        ).unwrap();
        
        manager.define_global_variable(
            "y".to_string(),
            Expression::Number(Number::from(20)),
            cell_id
        ).unwrap();
        
        let vars = manager.export_for_computation();
        assert_eq!(vars.len(), 2);
        assert!(vars.contains_key("x"));
        assert!(vars.contains_key("y"));
    }
}