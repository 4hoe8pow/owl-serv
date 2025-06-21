use crate::value_objects::{
    department_id::DepartmentId, department_name::DepartmentName,
    parent_department_id::ParentDepartmentId,
};

pub struct Department {
    pub department_id: DepartmentId,
    pub department_name: DepartmentName,
    pub parent_department_id: Option<ParentDepartmentId>,
}
