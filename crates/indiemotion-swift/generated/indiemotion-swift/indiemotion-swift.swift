
public class Name: NameRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Name$_free(ptr)
        }
    }
}
public class NameRefMut: NameRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class NameRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Name: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Name$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Name$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Name) {
        __swift_bridge__$Vec_Name$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Name$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Name(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<NameRef> {
        let pointer = __swift_bridge__$Vec_Name$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return NameRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<NameRefMut> {
        let pointer = __swift_bridge__$Vec_Name$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return NameRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Name$len(vecPtr)
    }
}


public class Matrix44: Matrix44RefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Matrix44$_free(ptr)
        }
    }
}
public class Matrix44RefMut: Matrix44Ref {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class Matrix44Ref {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Matrix44: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Matrix44$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Matrix44$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Matrix44) {
        __swift_bridge__$Vec_Matrix44$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Matrix44$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Matrix44(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Matrix44Ref> {
        let pointer = __swift_bridge__$Vec_Matrix44$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return Matrix44Ref(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Matrix44RefMut> {
        let pointer = __swift_bridge__$Vec_Matrix44$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return Matrix44RefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Matrix44$len(vecPtr)
    }
}


public class Vec4: Vec4RefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Vec4$_free(ptr)
        }
    }
}
public class Vec4RefMut: Vec4Ref {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class Vec4Ref {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Vec4: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Vec4$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Vec4$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Vec4) {
        __swift_bridge__$Vec_Vec4$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Vec4$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Vec4(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Vec4Ref> {
        let pointer = __swift_bridge__$Vec_Vec4$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return Vec4Ref(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Vec4RefMut> {
        let pointer = __swift_bridge__$Vec_Vec4$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return Vec4RefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Vec4$len(vecPtr)
    }
}


public class Vec3: Vec3RefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Vec3$_free(ptr)
        }
    }
}
public class Vec3RefMut: Vec3Ref {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class Vec3Ref {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Vec3: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Vec3$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Vec3$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Vec3) {
        __swift_bridge__$Vec_Vec3$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Vec3$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Vec3(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Vec3Ref> {
        let pointer = __swift_bridge__$Vec_Vec3$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return Vec3Ref(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<Vec3RefMut> {
        let pointer = __swift_bridge__$Vec_Vec3$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return Vec3RefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Vec3$len(vecPtr)
    }
}


public class Value: ValueRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$Value$_free(ptr)
        }
    }
}
public class ValueRefMut: ValueRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ValueRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension Value: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_Value$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_Value$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: Value) {
        __swift_bridge__$Vec_Value$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_Value$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (Value(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ValueRef> {
        let pointer = __swift_bridge__$Vec_Value$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ValueRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ValueRefMut> {
        let pointer = __swift_bridge__$Vec_Value$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ValueRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_Value$len(vecPtr)
    }
}


public class PropertyDef: PropertyDefRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$PropertyDef$_free(ptr)
        }
    }
}
extension PropertyDef {
    class public func new<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString, _ default_value: Value) -> PropertyDef {
        PropertyDef(ptr: __swift_bridge__$PropertyDef$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), {default_value.isOwned = false; return default_value.ptr;}()))
    }
}
public class PropertyDefRefMut: PropertyDefRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class PropertyDefRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension PropertyDefRef {
    public func name() -> NameRef {
        NameRef(ptr: __swift_bridge__$PropertyDef$name(ptr))
    }

    public func default_value() -> ValueRef {
        ValueRef(ptr: __swift_bridge__$PropertyDef$default_value(ptr))
    }
}
extension PropertyDef: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_PropertyDef$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_PropertyDef$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: PropertyDef) {
        __swift_bridge__$Vec_PropertyDef$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_PropertyDef$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (PropertyDef(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PropertyDefRef> {
        let pointer = __swift_bridge__$Vec_PropertyDef$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PropertyDefRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<PropertyDefRefMut> {
        let pointer = __swift_bridge__$Vec_PropertyDef$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return PropertyDefRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_PropertyDef$len(vecPtr)
    }
}


public class ControllerDef: ControllerDefRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$ControllerDef$_free(ptr)
        }
    }
}
extension ControllerDef {
    class public func new<GenericIntoRustString: IntoRustString>(_ name: GenericIntoRustString, _ properties: RustVec<PropertyDef>) -> ControllerDef {
        ControllerDef(ptr: __swift_bridge__$ControllerDef$new({ let rustString = name.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let val = properties; val.isOwned = false; return val.ptr }()))
    }
}
public class ControllerDefRefMut: ControllerDefRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class ControllerDefRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension ControllerDefRef {
    public func name() -> NameRef {
        NameRef(ptr: __swift_bridge__$ControllerDef$name(ptr))
    }

    public func property(_ name: NameRef) -> Optional<PropertyDefRef> {
        { let val = __swift_bridge__$ControllerDef$property(ptr, name.ptr); if val != nil { return PropertyDef(ptr: val!) } else { return nil } }()
    }
}
extension ControllerDef: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_ControllerDef$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_ControllerDef$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: ControllerDef) {
        __swift_bridge__$Vec_ControllerDef$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_ControllerDef$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (ControllerDef(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ControllerDefRef> {
        let pointer = __swift_bridge__$Vec_ControllerDef$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ControllerDefRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<ControllerDefRefMut> {
        let pointer = __swift_bridge__$Vec_ControllerDef$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return ControllerDefRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_ControllerDef$len(vecPtr)
    }
}



