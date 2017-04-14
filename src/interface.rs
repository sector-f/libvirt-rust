/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <http://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

#![allow(improper_ctypes)]

extern crate libc;

use std::ffi::{CString, CStr};
use std::str;

use connect::{Connect, virConnectPtr};
use error::Error;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct virInterface {
}

#[allow(non_camel_case_types)]
pub type virInterfacePtr = *const virInterface;

#[link(name = "virt")]
extern {
    fn virInterfaceLookupByID(c: virConnectPtr,
                              id: libc::c_int) -> virInterfacePtr;
    fn virInterfaceLookupByName(c: virConnectPtr,id: *const libc::c_char) -> virInterfacePtr;
    fn virInterfaceLookupByMACString(c: virConnectPtr,id: *const libc::c_char) -> virInterfacePtr;
    fn virInterfaceLookupByUUIDString(c: virConnectPtr, uuid: *const libc::c_char) -> virInterfacePtr;
    fn virInterfaceDefineXML(c: virConnectPtr, xml: *const libc::c_char, flags: libc::c_uint) -> virInterfacePtr;
    fn virInterfaceCreate(d: virInterfacePtr, flags: libc::c_uint) -> libc::c_int;
    fn virInterfaceDestroy(d: virInterfacePtr) -> libc::c_int;
    fn virInterfaceUndefine(d: virInterfacePtr) -> libc::c_int;
    fn virInterfaceFree(d: virInterfacePtr) -> libc::c_int;
    fn virInterfaceIsActive(d: virInterfacePtr) -> libc::c_int;
    fn virInterfaceGetName(d: virInterfacePtr) -> *const libc::c_char;
    fn virInterfaceGetMACString(d: virInterfacePtr) -> *const libc::c_char;
    fn virInterfaceGetXMLDesc(d: virInterfacePtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virInterfaceGetUUIDString(d: virInterfacePtr, uuid: *mut libc::c_char) -> libc::c_int;
    fn virInterfaceGetConnect(d: virInterfacePtr) -> virConnectPtr;

    // TODO: need to be implemented
    fn virInterfaceChangeBegin() -> ();
    fn virInterfaceRef() -> ();
    fn virInterfaceChangeRollback() -> ();
    fn virInterfaceChangeCommit() -> ();
}

pub type InterfaceXMLFlags = self::libc::c_uint;
pub const VIR_INTERFACE_XML_INACTIVE:InterfaceXMLFlags = 1 << 0;

pub struct Interface {
    pub d: virInterfacePtr
}

impl Interface {

    pub fn as_ptr(&self) -> virInterfacePtr {
        self.d
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virInterfaceGetConnect(self.d);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect{c: ptr});
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface{d: ptr});
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByName(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface{d: ptr});
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str, flags: u32) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceDefineXML(
                conn.as_ptr(), CString::new(xml).unwrap().as_ptr(),
                flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface{d: ptr});
        }
    }

    pub fn lookup_by_mac_string(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByMACString(
                conn.as_ptr(), CString::new(id).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface{d: ptr});
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = virInterfaceLookupByUUIDString(
                conn.as_ptr(), CString::new(uuid).unwrap().as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Interface{d: ptr});
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virInterfaceGetName(self.d);
            if n.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(n).to_string_lossy().into_owned())
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virInterfaceGetUUIDString(self.d, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(
                uuid.as_ptr()).to_string_lossy().into_owned())
        }
    }

    pub fn get_mac_string(&self) -> Result<String, Error> {
        unsafe {
            let mac = virInterfaceGetMACString(self.d);
            if mac.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(mac).to_string_lossy().into_owned())
        }
    }

    pub fn get_xml_desc(&self, flags:InterfaceXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = virInterfaceGetXMLDesc(self.d, flags);
            if xml.is_null() {
                return Err(Error::new())
            }
            return Ok(CStr::from_ptr(xml).to_string_lossy().into_owned())
        }
    }

    pub fn create(&self, flags: InterfaceXMLFlags) -> Result<(), Error> {
        unsafe {
            if virInterfaceCreate(self.d, flags) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceDestroy(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceUndefine(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&self) -> Result<(), Error> {
        unsafe {
            if virInterfaceFree(self.d) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virInterfaceIsActive(self.d);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }
}
