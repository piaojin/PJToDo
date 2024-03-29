//
//  PJHttpReponseStatusCode.swift
//  PJToDo
//
//  Created by piaojin on 2019/1/26.
//  Copyright © 2019 piaojin. All rights reserved.
//

import UIKit

public enum PJHttpReponseStatusCode: Int {
    //The request can be continued.
    case HTTP_STATUS_CONTINUE = 100
    
    //The server has switched protocols in an upgrade header.
    case HTTP_STATUS_SWITCH_PROTOCOLS = 101
    
    //The request completed successfully.
    case HTTP_STATUS_OK = 200
    
    //The request has been fulfilled and resulted in the creation of a new resource.
    case HTTP_STATUS_CREATED = 201
    
    //The request has been accepted for processing, but the processing has not been completed.
    case HTTP_STATUS_ACCEPTED = 202
    
    //The returned meta information in the entity-header is not the definitive set available from the origin server.
    case HTTP_STATUS_PARTIAL = 203
    
    //The server has fulfilled the request, but there is no new information to send back.
    case HTTP_STATUS_NO_CONTENT = 204
    
    //The request has been completed, and the client program should reset the document view that caused the request to be sent to allow the user to easily initiate another input action.
    case HTTP_STATUS_RESET_CONTENT = 205
    
    //The server has fulfilled the partial GET request for the resource.
    case HTTP_STATUS_PARTIAL_CONTENT = 206
    
    //The server couldn't decide what to return.
    case HTTP_STATUS_AMBIGUOUS = 300
    
    //The requested resource has been assigned to a new permanent URI (Uniform Resource Identifier), and any future references to this resource should be done using one of the returned URIs.
    case HTTP_STATUS_MOVED = 301
    
    //The requested resource resides temporarily under a different URI (Uniform Resource Identifier).
    case HTTP_STATUS_REDIRECT = 302
    
    //The response to the request can be found under a different URI (Uniform Resource Identifier) and should be retrieved using a GET HTTP verb on that resource.
    case HTTP_STATUS_REDIRECT_METHOD = 303
    
    //The requested resource has not been modified.
    case HTTP_STATUS_NOT_MODIFIED = 304
    
    //The requested resource must be accessed through the proxy given by the location field.
    case HTTP_STATUS_USE_PROXY = 305
    
    //The redirected request keeps the same HTTP verb. HTTP/1.1 behavior.
    case HTTP_STATUS_REDIRECT_KEEP_VERB = 307
    
    //The request could not be processed by the server due to invalid syntax.
    case HTTP_STATUS_BAD_REQUEST = 400
    
    //The requested resource requires user authentication.
    case HTTP_STATUS_DENIED = 401
    
    //Not currently implemented in the HTTP protocol.
    case HTTP_STATUS_PAYMENT_REQ = 402
    
    //The server understood the request, but is refusing to fulfill it.
    case HTTP_STATUS_FORBIDDEN = 403
    
    //The server has not found anything matching the requested URI (Uniform Resource Identifier).
    case HTTP_STATUS_NOT_FOUND = 404
    
    //The HTTP verb used is not allowed.
    case HTTP_STATUS_BAD_METHOD = 405
    
    //No responses acceptable to the client were found.
    case HTTP_STATUS_NONE_ACCEPTABLE  = 406
    
    //Proxy authentication required.
    case HTTP_STATUS_PROXY_AUTH_REQ = 407
    
    //The server timed out waiting for the request.
    case HTTP_STATUS_REQUEST_TIMEOUT = 408
    
    //The request could not be completed due to a conflict with the current state of the resource. The user should resubmit with more information.
    case HTTP_STATUS_CONFLICT = 409
    
    //The requested resource is no longer available at the server, and no forwarding address is known.
    case HTTP_STATUS_GONE = 410
    
    //The server refuses to accept the request without a defined content length.
    case HTTP_STATUS_LENGTH_REQUIRED = 411
    
    //The precondition given in one or more of the request header fields evaluated to false when it was tested on the server.
    case HTTP_STATUS_PRECOND_FAILED = 412
    
    //The server is refusing to process a request because the request entity is larger than the server is willing or able to process.
    case HTTP_STATUS_REQUEST_TOO_LARGE = 413
    
    //The server is refusing to service the request because the request URI (Uniform Resource Identifier) is longer than the server is willing to interpret.
    case HTTP_STATUS_URI_TOO_LONG = 414
    
    //The server is refusing to service the request because the entity of the request is in a format not supported by the requested resource for the requested method.
    case HTTP_STATUS_UNSUPPORTED_MEDIA = 415
    
    //Unprocessable Entity
    case HTTP_STATUS_UNPROCESSABLE_ENTITY = 422
    
    //The request should be retried after doing the appropriate action.
    case HTTP_STATUS_RETRY_WITH = 449
    
    //The server encountered an unexpected condition that prevented it from fulfilling the request.
    case HTTP_STATUS_SERVER_ERROR = 500
    
    //The server does not support the functionality required to fulfill the request.
    case HTTP_STATUS_NOT_SUPPORTED = 501
    
    //The server, while acting as a gateway or proxy, received an invalid response from the upstream server it accessed in attempting to fulfill the request.
    case HTTP_STATUS_BAD_GATEWAY = 502
    
    //The service is temporarily overloaded.
    case HTTP_STATUS_SERVICE_UNAVAIL = 503
    
    //The request was timed out waiting for a gateway.
    case HTTP_STATUS_GATEWAY_TIMEOUT = 504
    
    //The server does not support, or refuses to support, the HTTP protocol version that was used in the request message.
    case HTTP_STATUS_VERSION_NOT_SUP = 505
}
