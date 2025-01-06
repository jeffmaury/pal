# \ModelsApi

All URIs are relative to *http://127.0.0.1:10434*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_models**](ModelsApi.md#get_models) | **GET** /api/tags | List models that are available locally
[**pull_model**](ModelsApi.md#pull_model) | **POST** /api/pull | Download a model from the Podman AI Lab Catalog. 



## get_models

> models::ListResponse get_models()
List models that are available locally

List models that are available locally

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListResponse**](ListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_model

> models::ProgressResponse pull_model(pull_request)
Download a model from the Podman AI Lab Catalog. 

Download a model from the Podman AI Lab catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_request** | [**PullRequest**](PullRequest.md) | Request to pull a model | [required] |

### Return type

[**models::ProgressResponse**](ProgressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

