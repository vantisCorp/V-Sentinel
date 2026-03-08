package vsentinel

import (
	"context"
)

// ThreatIntelService handles threat intelligence API calls.
type ThreatIntelService struct {
	client *Client
}

// IOCListOptions represents options for listing IOCs.
type IOCListOptions struct {
	Type          IOCType
	Confidence    IOCConfidence
	ThreatActor   string
	MalwareFamily string
	Tags          []string
	TimeRange     string
	SortBy        string
	SortOrder     string
	Limit         int
	Offset        int
}

// toParams converts options to query parameters.
func (o *IOCListOptions) toParams() map[string]string {
	params := make(map[string]string)

	if o.Type != "" {
		params["type"] = string(o.Type)
	}
	if o.Confidence != "" {
		params["confidence"] = string(o.Confidence)
	}
	if o.ThreatActor != "" {
		params["threat_actor"] = o.ThreatActor
	}
	if o.MalwareFamily != "" {
		params["malware_family"] = o.MalwareFamily
	}
	if len(o.Tags) > 0 {
		params["tags"] = joinStrings(o.Tags, ",")
	}
	if o.TimeRange != "" {
		params["time_range"] = o.TimeRange
	}
	if o.SortBy != "" {
		params["sort_by"] = o.SortBy
	}
	if o.SortOrder != "" {
		params["sort_order"] = o.SortOrder
	}
	if o.Limit > 0 {
		params["limit"] = intToString(o.Limit)
	}
	if o.Offset > 0 {
		params["offset"] = intToString(o.Offset)
	}

	return params
}

// ListIOCs retrieves a list of IOCs.
func (s *ThreatIntelService) ListIOCs(ctx context.Context, opts *IOCListOptions) (*IOCList, error) {
	var result IOCList

	params := make(map[string]string)
	if opts != nil {
		params = opts.toParams()
	}

	if _, ok := params["limit"]; !ok {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/threat-intel/iocs", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// GetIOC retrieves a specific IOC by ID.
func (s *ThreatIntelService) GetIOC(ctx context.Context, iocID string) (*IOC, error) {
	var result IOC

	err := s.client.get(ctx, "/threat-intel/iocs/"+iocID, nil, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// CheckIOCOptions represents options for checking an IOC.
type CheckIOCOptions struct {
	Value string
	Type  IOCType
}

// CheckIOC checks if an indicator is malicious.
func (s *ThreatIntelService) CheckIOC(ctx context.Context, value string, iocType IOCType) (*IOCCheckResult, error) {
	var result IOCCheckResult

	params := map[string]string{"value": value}
	if iocType != "" {
		params["type"] = string(iocType)
	}

	err := s.client.get(ctx, "/threat-intel/check", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// CheckIOCsBulk checks multiple IOCs in bulk.
func (s *ThreatIntelService) CheckIOCsBulk(ctx context.Context, values []string, iocType IOCType) ([]IOCCheckResult, error) {
	var result struct {
		Results []IOCCheckResult `json:"results"`
	}

	body := map[string]any{"values": values}
	if iocType != "" {
		body["type"] = string(iocType)
	}

	err := s.client.post(ctx, "/threat-intel/check/bulk", body, &result)
	if err != nil {
		return nil, err
	}

	return result.Results, nil
}

// AddIOCOptions represents options for adding an IOC.
type AddIOCOptions struct {
	Value           string          `json:"value"`
	Type            IOCType         `json:"type"`
	Confidence      IOCConfidence   `json:"confidence"`
	ThreatActor     string          `json:"threat_actor,omitempty"`
	Campaign        string          `json:"campaign,omitempty"`
	MalwareFamily   string          `json:"malware_family,omitempty"`
	Tags            []string        `json:"tags,omitempty"`
	Description     string          `json:"description,omitempty"`
	References      []string        `json:"references,omitempty"`
	ExpirationDays  int             `json:"expiration_days,omitempty"`
}

// AddIOC adds a new IOC to the database.
func (s *ThreatIntelService) AddIOC(ctx context.Context, opts *AddIOCOptions) (*IOC, error) {
	var result IOC

	err := s.client.post(ctx, "/threat-intel/iocs", opts, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// UpdateIOCOptions represents options for updating an IOC.
type UpdateIOCOptions struct {
	Confidence    IOCConfidence `json:"confidence,omitempty"`
	ThreatActor   string        `json:"threat_actor,omitempty"`
	Campaign      string        `json:"campaign,omitempty"`
	MalwareFamily string        `json:"malware_family,omitempty"`
	Tags          []string      `json:"tags,omitempty"`
	Description   string        `json:"description,omitempty"`
}

// UpdateIOC updates an existing IOC.
func (s *ThreatIntelService) UpdateIOC(ctx context.Context, iocID string, opts *UpdateIOCOptions) (*IOC, error) {
	var result IOC

	err := s.client.patch(ctx, "/threat-intel/iocs/"+iocID, opts, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// DeleteIOC deletes an IOC.
func (s *ThreatIntelService) DeleteIOC(ctx context.Context, iocID string) error {
	return s.client.delete(ctx, "/threat-intel/iocs/"+iocID, nil, nil)
}

// SearchIOCs searches for IOCs matching a query.
func (s *ThreatIntelService) SearchIOCs(ctx context.Context, query string, limit int) (*IOCList, error) {
	var result IOCList

	params := map[string]string{"query": query}
	if limit > 0 {
		params["limit"] = intToString(limit)
	} else {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/threat-intel/iocs/search", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// ThreatActorListOptions represents options for listing threat actors.
type ThreatActorListOptions struct {
	Country    string
	Motivation string
	Tags       []string
	Limit      int
	Offset     int
}

// toParams converts options to query parameters.
func (o *ThreatActorListOptions) toParams() map[string]string {
	params := make(map[string]string)

	if o.Country != "" {
		params["country"] = o.Country
	}
	if o.Motivation != "" {
		params["motivation"] = o.Motivation
	}
	if len(o.Tags) > 0 {
		params["tags"] = joinStrings(o.Tags, ",")
	}
	if o.Limit > 0 {
		params["limit"] = intToString(o.Limit)
	}
	if o.Offset > 0 {
		params["offset"] = intToString(o.Offset)
	}

	return params
}

// ListThreatActors retrieves a list of threat actors.
func (s *ThreatIntelService) ListThreatActors(ctx context.Context, opts *ThreatActorListOptions) (*ThreatActorList, error) {
	var result ThreatActorList

	params := make(map[string]string)
	if opts != nil {
		params = opts.toParams()
	}

	if _, ok := params["limit"]; !ok {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/threat-intel/actors", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// GetThreatActor retrieves a specific threat actor by ID or name.
func (s *ThreatIntelService) GetThreatActor(ctx context.Context, actorID string) (*ThreatActor, error) {
	var result ThreatActor

	err := s.client.get(ctx, "/threat-intel/actors/"+actorID, nil, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// GetThreatActorIOCs retrieves IOCs associated with a threat actor.
func (s *ThreatIntelService) GetThreatActorIOCs(ctx context.Context, actorID string, limit int) (*IOCList, error) {
	var result IOCList

	params := map[string]string{}
	if limit > 0 {
		params["limit"] = intToString(limit)
	} else {
		params["limit"] = "100"
	}

	err := s.client.get(ctx, "/threat-intel/actors/"+actorID+"/iocs", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// CampaignListOptions represents options for listing campaigns.
type CampaignListOptions struct {
	ThreatActor string
	Status      string
	TimeRange   string
	Limit       int
	Offset      int
}

// toParams converts options to query parameters.
func (o *CampaignListOptions) toParams() map[string]string {
	params := make(map[string]string)

	if o.ThreatActor != "" {
		params["threat_actor"] = o.ThreatActor
	}
	if o.Status != "" {
		params["status"] = o.Status
	}
	if o.TimeRange != "" {
		params["time_range"] = o.TimeRange
	}
	if o.Limit > 0 {
		params["limit"] = intToString(o.Limit)
	}
	if o.Offset > 0 {
		params["offset"] = intToString(o.Offset)
	}

	return params
}

// ListCampaigns retrieves a list of campaigns.
func (s *ThreatIntelService) ListCampaigns(ctx context.Context, opts *CampaignListOptions) (*CampaignList, error) {
	var result CampaignList

	params := make(map[string]string)
	if opts != nil {
		params = opts.toParams()
	}

	if _, ok := params["limit"]; !ok {
		params["limit"] = "50"
	}

	err := s.client.get(ctx, "/threat-intel/campaigns", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// GetCampaign retrieves a specific campaign by ID.
func (s *ThreatIntelService) GetCampaign(ctx context.Context, campaignID string) (*Campaign, error) {
	var result Campaign

	err := s.client.get(ctx, "/threat-intel/campaigns/"+campaignID, nil, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// GetCampaignIOCs retrieves IOCs associated with a campaign.
func (s *ThreatIntelService) GetCampaignIOCs(ctx context.Context, campaignID string, limit int) (*IOCList, error) {
	var result IOCList

	params := map[string]string{}
	if limit > 0 {
		params["limit"] = intToString(limit)
	} else {
		params["limit"] = "100"
	}

	err := s.client.get(ctx, "/threat-intel/campaigns/"+campaignID+"/iocs", params, &result)
	if err != nil {
		return nil, err
	}

	return &result, nil
}

// ExportOptions represents options for exporting IOCs.
type ExportOptions struct {
	Type           IOCType
	ThreatActor    string
	MalwareFamily  string
	Tags           []string
	Format         string
	IncludeExpired bool
}

// Export exports IOCs in the specified format.
func (s *ThreatIntelService) Export(ctx context.Context, opts *ExportOptions) (string, error) {
	params := map[string]string{
		"format": opts.Format,
	}

	if opts.Type != "" {
		params["type"] = string(opts.Type)
	}
	if opts.ThreatActor != "" {
		params["threat_actor"] = opts.ThreatActor
	}
	if opts.MalwareFamily != "" {
		params["malware_family"] = opts.MalwareFamily
	}
	if len(opts.Tags) > 0 {
		params["tags"] = joinStrings(opts.Tags, ",")
	}
	if opts.IncludeExpired {
		params["include_expired"] = "true"
	}

	var result string

	err := s.client.get(ctx, "/threat-intel/export", params, &result)
	if err != nil {
		return "", err
	}

	return result, nil
}

// ExportForSIEM exports IOCs formatted for a specific SIEM.
func (s *ThreatIntelService) ExportForSIEM(ctx context.Context, siemType string, iocTypes []IOCType, threatActor string) (string, error) {
	params := map[string]string{"siem": siemType}

	if len(iocTypes) > 0 {
		params["types"] = joinIOCType(iocTypes, ",")
	}
	if threatActor != "" {
		params["threat_actor"] = threatActor
	}

	var result string

	err := s.client.get(ctx, "/threat-intel/export/siem", params, &result)
	if err != nil {
		return "", err
	}

	return result, nil
}