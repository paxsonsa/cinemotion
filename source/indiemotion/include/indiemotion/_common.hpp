// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* _common.hpp 

Header for providing common include across the library
*/
#pragma once
#include <any>
#include <iostream>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <boost/format.hpp>


template<typename TO, typename FROM>
std::unique_ptr<TO> static_unique_pointer_cast (std::unique_ptr<FROM>&& old){
    return std::unique_ptr<TO>{static_cast<TO*>(old.release())};
    //conversion: unique_ptr<FROM>->FROM*->TO*->unique_ptr<TO>
}