// Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
/* kind.hpp */
#pragma once

namespace indiemotion::responses
{
    /**
     * @brief Represents the kind of messages available
     */
    enum class Kind
    {
        Acknowledgment = 0,
        Error = 1,
        InitSession = 100,
        CameraList = 200
    };

    struct KindNames
    {
        inline static const std::string Acknowledgment = "Acknowledgment";
        inline static const std::string Error = "Error";
        inline static const std::string InitSession = "InitSession";
        inline static const std::string CameraList = "CameraList";
    };

    /**
     * @brief return a string name for the given kind
     * 
     * @param k the kind to transform into string
     * @return std::string 
     */
    std::string
    kindToStr(Kind k)
    {
        switch (k)
        {
        case Kind::Acknowledgment:
            return KindNames::Acknowledgment;
        case Kind::Error:
            return KindNames::Error;
        case Kind::InitSession:
            return KindNames::InitSession;
        case Kind::CameraList:
            return KindNames::CameraList;
        }
    }
}