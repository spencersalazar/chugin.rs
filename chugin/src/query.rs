
use crate::chuck;
use crate::cstring::CString as CString;
use crate::CKResult;

/// Chugin Query wrapper class
pub struct Query {
    query: *mut chuck::DL_Query,
}

/// Chugin Query wrapper class
impl Query {
    
    /// Create new wrapper from ChucK type
    pub fn new(query: *mut chuck::DL_Query) -> CKResult<Query> {
        if !query.is_null() {
            Ok(Query { query: query })
        } else {
            Err("invalid query object provided")
        }
    }
    
    /// Begin a new class
    pub fn begin_class(&self, name: &str, parent: &str) -> CKResult {
        
        let name = CString::new(name)?;
        let parent = CString::new(parent)?;
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let begin_class = match query.begin_class {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            begin_class(self.query, name.c_str(), parent.c_str());
        }
        
        Ok(())
    }
    
    /// Add a constructor for the class that is currently being constructed
    pub fn add_ctor(&self, ctor: chuck::f_ctor) -> CKResult {
                
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_ctor = match query.add_ctor {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            add_ctor(self.query, ctor);
        }
        
        Ok(())
    }
    
    /// Add a destructor for the class that is currently being constructed
    pub fn add_dtor(&self, dtor: chuck::f_dtor) -> CKResult {
                
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_dtor = match query.add_dtor {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            add_dtor(self.query, dtor);
        }
        
        Ok(())
    }
    
    /// Add a member variable for the class that is being constructed
    pub fn add_mvar(&self, type_: &str, name: &str, is_const: bool) -> CKResult<chuck::t_CKUINT> {
        
        let type_ = CString::new(type_)?;
        let name = CString::new(name)?;
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_mvar = match query.add_mvar {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        Ok(unsafe {
            add_mvar(self.query, type_.c_str(), name.c_str(), if is_const { 1 } else { 0 })
        })
    }
    
    /// Add a tick function for the class that is being constructed
    pub fn add_mfun(&self, 
        mfun: chuck::f_mfun, 
        type_: &str, name: &str,
        args: &[(String,String)]
    ) -> CKResult {
        
        let type_ = CString::new(type_)?;
        let name = CString::new(name)?;
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_mfun = match query.add_mfun {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        let add_arg = match query.add_arg {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            add_mfun(self.query, mfun, type_.c_str(), name.c_str());
        }
        
        for arg in args {
            let type_ = CString::new(&arg.0)?;
            let name = CString::new(&arg.1)?;
            
            unsafe {
                add_arg(self.query, type_.c_str(), name.c_str());
            }
        }
        
        Ok(())
    }
    
    /// Add a tick function for the class that is being constructed
    pub fn add_ugen_func(&self, tick: chuck::f_tick, num_in: u32, num_out: u32) -> CKResult {
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let add_ugen_func = match query.add_ugen_func {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        unsafe {
            add_ugen_func(self.query, tick, None, num_in.into(), num_out.into());
        }
        
        Ok(())
    }
    
    /// End a class that is being constructed
    pub fn end_class(&self) -> CKResult {
        
        let query = match unsafe { self.query.as_ref() } {
            Some(query) => query,
            None => return Err("invalid query object")
        };
        
        let end_class = match query.end_class {
            Some(f) => f,
            None => return Err("invalid query object"),
        };
        
        match unsafe { end_class(self.query) } {
            0 => Err("failed to end_class"),
            _ => Ok(()),
        }
    }
}
